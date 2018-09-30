import Foundation


#if os(Linux)
    srandom(UInt32(time(nil)))
#endif


func getRandomNum() -> UInt64 {
    #if os(Linux)
        return UInt64(random())
    #else
        return UInt64(arc4random_uniform(UInt32.max))
    #endif
}


func f(_ x: Double) -> Double {
	return 6*sin(0.01*x) + 3 + 2*(Double(getRandomNum()) / Double(UInt32.max)) - 1
}
func genJSONFile(_ size: Int, useArrays: Bool) {
	let pairs: Any
	if useArrays {
		pairs = (0..<size).map(Double.init).map(f).enumerated().map { [$0.0, $0.1] }
	} else {
		pairs = (0..<size).map(Double.init).map(f).enumerated().map { ["time" : $0.0, "voltage" : $0.1] }
	}

	let prefixPath = useArrays ? "json/arrays/" : "json/objects/"

	let os: OutputStream = OutputStream.init(toFileAtPath: "\(prefixPath)data_\(size).json", append: false)!
	os.open()

	print("Writing JSON data...")
	let _ = try! JSONSerialization.writeJSONObject(pairs, toStream: os, options: [])
	os.close()

	print()
}


func genCSVFile(_ size: Int) {
	let pairs = (0..<size).map(Double.init).map(f).enumerated().map { ($0.0, $0.1) }

	let prefixPath = "csv/"

	let outUrl = URL(fileURLWithPath: "\(prefixPath)data_\(size).csv")

	print("Writing CSV data...")
	
	let csvText = "time,voltage\n" + pairs.map { "\($0.0),\($0.1)" }.joined(separator: "\n")

	try! csvText.write(to: outUrl, atomically: false, encoding: .ascii)

	print()
}

// 40
for i in 0..<35 {
	let size = Int(pow(1.6, Double(i)))
	print("(\(i)) Generating data with \(size) data points...")
	// genJSONFile(size, useArrays: true)
	genCSVFile(size)
}