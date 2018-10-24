
NameIdTime = Struct.new(:name, :execId, :gcfTime)
RunData = Struct.new(:endToEnd, :functionData)
NameTime = Struct.new(:name, :gcfTime)

def run_once(llsplName, input = "{}")
    execIdsAndNames = []

    # Start spl-lib from another thread
    puts "Starting spl-lib..."
    receivedError = false
    t = Thread.start do
        IO.popen(["../target/release/spl-lib", "-d", llsplName]) { |f|
            while true do
                splLine = f.gets.chomp
                pair = splLine.split(",", 3)
                if pair.length == 2 and pair[0] == "@json"
                    puts "Received json time from spl-lib: #{pair[1]} ms for #{pair[2]}"
                    execIdsAndNames.push(pair)
                elsif pair.length == 3 and pair[0] == "@download"
                    puts "Received download time from spl-lib: #{pair[1]} ms for #{pair[2]}"
                    execIdsAndNames.push(pair)
                elsif pair.length == 2
                    if pair[0] == "ERROR"
                        puts "Received error from spl-lib for function #{pair[1]}"
                        receivedError = true
                    else
                        puts "Received exec id from spl-lib: #{pair}"
                        execIdsAndNames.push(pair)
                    end
                else
                    puts "Received spl-lib junk: #{splLine}"
                end
            end
        }
    end

    # Wait long enough for spl-lib to definitely start
    `sleep 1`

    # Run and time curl command
    puts "Starting curl command to connect to spl-lib"
    startTime = Time.now
    `curl -s -d '#{input}' localhost:8000/#{llsplName}`
    endTime = Time.now
    endToEnd = endTime - startTime

    # `sleep 2`

    if receivedError 
        # Cleanup
        t.kill
        `pkill spl-lib`
        return nil
    end

    # Scrape the logs for each exec id
    results = []

    for execIdAndName in execIdsAndNames
        if execIdAndName.length == 3 and execIdAndName[0] == "@download"
            dt = execIdAndName[1].to_f / 1000.0
            url = execIdAndName[2]
            results.push(NameIdTime.new("download " + url, "@download", dt))
            next
        elsif execIdAndName.length == 2 and execIdAndName[0] == "@json"
            dt = execIdAndName[1].to_f / 1000.0
            results.push(NameIdTime.new("json transform", "@json", dt))
            next
        end

        execId = execIdAndName[0]
        name = execIdAndName[1]

        # We loop until we successfully read the log of execId, in case it takes some time for the logs to appear.
        puts "Waiting to read exec id #{execId} from GCF logs..."
        while true
            command = "gcloud functions logs read --execution-id=#{execId}"
            logData = `#{command}`
            if match = logData.match(/([0-9.]+) ms/)
                gcfTime = match.captures[0].to_f / 1000.0
                results.push(NameIdTime.new(name, execId, gcfTime))
                break
            end
        end
    end
    
    # Cleanup
    t.kill
    `pkill spl-lib`

    RunData.new(endToEnd, results)
end

def run_many(llsplName, csvFilename, trials = 10, input = "{}")
    allData = []
    # trials.times do |n|
    #     print "\n\nRun #{n+1} / #{trials}\n=============\n\n"
    #     runData.push(run_once(llsplName, input))
    # end

    # Execute trials number of tests, retrying as needed.
    i = 0
    while i < trials
        print "\n\nRun #{i+1} / #{trials}\n=============\n\n"
        data = run_once(llsplName, input)

        if data != nil
            allData.push(data)
            i += 1
        end
    end

    # Summarize the data
    # Struct.new(:name, :execId, :gcfTime)

    csvOutput = ""

    csvOutput = allData.map  { |runData|
        endToEnd = runData.endToEnd
        "#{endToEnd}," + runData.functionData.map { |d| "#{d.gcfTime}" }.join(",")
    }.join("\n")

    csvOutput = "End-to-end," + allData[0].functionData.map { |d| "#{d.name}" }.join(",") + "\n" + csvOutput

    open(csvFilename, 'w') { |f|
        f.puts csvOutput
    }
end

if ARGV.length < 2
    puts "Usage: ruby bench.rb llspl-name output-csv-file [numRuns = 10]"
    exit(1)
end

if ARGV.length == 3
    numRuns = ARGV[2].to_i
else
    numRuns = 10
end


`gcloud config set functions/region us-east1`
`pkill spl-lib`
print `cargo build --release`
print run_many(ARGV[0], ARGV[1], numRuns)