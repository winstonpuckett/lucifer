[Installation Instructions](https://lucifer.winstonpuckett.com/docs/installation/)

## 0.3.0

Performance expectations will no longer be set if not provided. 

In previous versions, if there was no performance expectation explicitly set, the performance would default to 1 second. However, there are many tools which don't have any performance metrics such as developer hotfix tools internal to a company. Some of these may be expected to take multiple hours.

It doesn't make sense to force a developer to do math around a tool's performance when they have no preference around how long it takes. It would be better to simply remove the feature.

## 0.2.1

Give better errors when the user doesn't have permission to create a specified output file.

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