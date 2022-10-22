[Installation Instructions](https://lucifer.winstonpuckett.com/docs/installation/)

## 0.2.0

Allow specification of the output file name using the output directory.

For instance:
```bash
# produces ./results/results.json
lucifer -o ./results

# produces ./results/xyz.json
lucifer -o ./results/xyz.json

# produces ./results/xyz.json/results.json
lucifer -o ./results/xyz.json/
```

Note: this is technically a breaking change because anyone relying on lucifer outputing to a folder ending in .json will now receive a file with that name instead.

## 0.1.15

Oops. Push release notes for 0.1.14.

## 0.1.14

Stops panic when a non-existent filepath is passed with the input directory option.

## 0.1.5-0.1.13

Fix deployment errors

## 0.1.4

Trigger CI/CD Differently

## 0.1.3

Update typo in readme 

## 0.1.2 and below

Support major functionality and use cases