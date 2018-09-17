# csv2json
Serverless function to convert CSV to JSON. Input is CSV, from a file or as a string 
from the command line. Output is the CSV data in JSON format.

# Examples

```bash
curl -X POST https://us-central1-cloudfun-mapreduce.cloudfunctions.net/csv2json_GCF -H 'Content-Type: text/csv' --data-binary @some_csv_file.csv
```
Where `some_csv_file.csv` is a file containing your CSV data.

```bash
curl -X POST https://us-central1-cloudfun-mapreduce.cloudfunctions.net/csv2json_GCF -H 'Content-Type: text/csv' -d "`echo -en 'a,b,c\n1,2,3'`"
```
Assuming your CSV data is `a,b,c\n1,2,3`.


