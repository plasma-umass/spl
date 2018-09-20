import Foundation

func genFile(_ size: Int, useArrays: Bool) {
	func f(_ x: Double) -> Double {
		return 6*sin(0.01*x) + 3 + 2*(Double(arc4random_uniform(UInt32.max)) / Double(UInt32.max)) - 1
	}

	let pairs: Any
	if useArrays {
		pairs = (0..<size).map(Double.init).map(f).enumerated().map { [$0.0, $0.1] }
	} else {
		pairs = (0..<size).map(Double.init).map(f).enumerated().map { ["time" : $0.0, "voltage" : $0.1] }
	}

	let prefixPath = useArrays ? "arrays/" : "objects/"

	let os: OutputStream = OutputStream.init(toFileAtPath: "\(prefixPath)data_\(size).json", append: false)!
	os.open()
	var err: NSError? = nil

	print("Writing JSON data...")
	JSONSerialization.writeJSONObject(pairs, to: os, error: &err)
	os.close()

	print()
}

// 40
for i in 0..<35 {
	let size = Int(pow(1.6, Double(i)))
	print("(\(i)) Generating data with \(size) data points...")
	genFile(size, useArrays: true)
}