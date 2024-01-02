# Email-Tool

This program requires a Google OAuth client to run. Additionally, your .env file needs to contain the following variables:
- SECRET_PATH: Path to your secret.json (OAuth)
- TEMPLATE_PATH: Path to the message template
- ANSWER_TEMPLATE_PATH: Path to the response template
- BEG: Initial separator
- END: Final separator

Variables in your templates need to be enclosed between BEG and END. For example, if BEG = '{' and END = '}' and a variable is named VARIABLE:
`Your variable is {VARIABLE}`

Finally, you can execute the program by passing the email subject you wish to process as an argument. For example:
`cargo run "Email Test"`
