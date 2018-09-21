def cargo_build
    `cargo build`
end


NameIdTime = Struct.new(:name, :execId, :gcfTime)

def run_once(llsplName, input = "{}")
    execIdsAndNames = []

    # Start spl-lib from another thread
    puts "Starting spl-lib..."
    t = Thread.start do
        IO.popen(["../target/debug/spl-lib", "-d", llsplName]) { |f|
            while true do
                pair = f.gets.chomp.split(",")
                if pair.length == 2
                    puts "Received exec id from spl-lib: #{pair}"
                    execIdsAndNames.push(pair)
                end
            end
        }
    end

    # Wait long enough for spl-lib to definitely start
    `sleep 2`

    # Run and time curl command
    puts "Starting curl command to connect to spl-lib"
    startTime = Time.now
    `curl -s -d '#{input}' localhost:8000/#{llsplName}`
    endTime = Time.now
    endToEndMs = (endTime - startTime) * 1000

    `sleep 2`

    # Scrape the logs for each exec id
    results = []

    for execIdAndName in execIdsAndNames
        execId = execIdAndName[0]
        name = execIdAndName[1]

        # We loop until we successfully read the log of execId, in case it takes some time for the logs to appear.
        puts "Waiting to read exec id #{execId} from GCF logs..."
        while true
            command = "gcloud functions logs read --execution-id=#{execId}"
            logData = `#{command}`
            if match = logData.match(/([0-9.]+) ms/)
                gcfTime = match.captures[0]
                results.push(NameIdTime.new(name, execId, gcfTime))
                break
            end
        end
    end
    
    # Cleanup
    t.kill
    `pkill spl-lib`

    results
end

def run_many(llsplName, input = "{}", trials = 10)
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

        if data.length > 0
            allData.push(data)
            i += 1
        end
    end

    # Summarize the data
    
    print runData
end

`pkill spl-lib`
print cargo_build
print run_many("download-census")