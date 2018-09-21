
NameIdTime = Struct.new(:name, :execId, :gcfTime)
RunData = Struct.new(:endToEnd, :functionData)
NameTime = Struct.new(:name, :gcfTime)

def run_once(llsplName, input = "{}")
    execIdsAndNames = []

    # Start spl-lib from another thread
    puts "Starting spl-lib..."
    receivedError = false
    t = Thread.start do
        IO.popen(["../target/debug/spl-lib", "-d", llsplName]) { |f|
            while true do
                splLine = f.gets.chomp
                pair = splLine.split(",")
                if pair.length == 2
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

def run_many(llsplName, trials = 10, input = "{}")
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

    endToEndSum = 0.0
    functionsSum = []
    puts ""
    # puts allData
    for runData in allData
        endToEndSum += runData.endToEnd

        if functionsSum == []
            functionsSum = runData.functionData
        else
            functionsSum = runData.functionData.zip(functionsSum).map { |functionRes, sumRes|
                if functionRes.name != sumRes.name
                    nil
                else
                    NameTime.new(functionRes.name, functionRes.gcfTime + sumRes.gcfTime)
                end
            }

            if functionsSum.include?(nil)
                puts "Error summing data..."
                break
            end
        end
    end

    endToEndMean = endToEndSum / allData.length
    functionsMean = functionsSum.map { |functionData|
        NameTime.new(functionData.name, functionData.gcfTime / allData.length)
    }

    puts ""
    puts "End to end mean: #{endToEndMean}\n\n"

    functionsMeanDesc = functionsMean.map { |data| "#{data.name},#{data.gcfTime}" }.join("\n")

    puts "Mean of each function:\n#{functionsMeanDesc}"
end

`gcloud config set functions/region us-east1`
`pkill spl-lib`
print `cargo build`
print run_many("download-census", 100)