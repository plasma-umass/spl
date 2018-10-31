if ARGV.length < 1
    puts "Usage: ruby csv-bench.rb numRunsEach"
    exit(1)
end

numRuns = ARGV[0].to_i

sizes = [10011,110908,1214683,13263266,133,144570688,1445,16228,178451,1950655,21287616,221,2322,26182,288092,3153887,33,34455485,344,3768,42713,467728,5112676,52,55796203,564,6083,68895,755117,8248065,894]
sizes.sort!

for s in sizes
    system("ruby bench.rb plot-case-study download-csv-benchmarks/data/data-#{s}.csv #{numRuns} 'https://people.cs.umass.edu/~dpinckney/test_plotting_data/csv/#{s}.csv'", out: $stdout, err: $stderr)
end