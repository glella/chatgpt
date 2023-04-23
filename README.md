# chatgpt

Query chatgpt from the command line. Written in Rust and using rustyline, reqwest, serde, dotenv & tokio.

Super simple: 
- Write your query from the prompt.
- Preserves history of requests.
- But chatgpt agent does not remember previous requests.


## Need OPEN_AI_API_KEY set in the enviroment variables:

If you don't have one request it at OpenAI

Then add to .bash_profile file like

```
export OPEN_API_API_KEY = xxx
```

or in fish shell:

```
set -x OPEN_AI_API_KEY xxx
```
