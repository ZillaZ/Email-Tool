# Email-Tool

[![Ubuntu](https://github.com/ZillaZ/Email-Tool/workflows/Ubuntu/badge.svg)](https://github.com/ZillaZ/Email-Tool/actions/workflows/ubuntu.yml)
[![Windows](https://github.com/ZillaZ/Email-Tool/workflows/Windows/badge.svg)](https://github.com/ZillaZ/Email-Tool/actions/workflows/windows.yml)

This program requires a Google OAuth client to run. Additionally, your `.env` file needs to contain the following variables:

- `SECRET_PATH`: Path to your `secret.json` (OAuth)
- `TEMPLATE_PATH`: Path to the message template
- `ANSWER_TEMPLATE_PATH`: Path to the response template
- `BEG`: Initial separator
- `END`: Final separator

Variables in your templates need to be enclosed between `BEG` and `END`. For example, if `BEG = '{'` and `END = '}'` and a variable is named `VARIABLE`:
`Your variable is {VARIABLE}`

Finally, you can execute the program by passing the email subject you wish to process as an argument. For example:
```bash
cargo run -- -s "Email Test"
```
You can use flags to identify different arguments.
- `-s "Subject": Identifies the email subject.`
- `-w true: The program will write the message on your template path.`