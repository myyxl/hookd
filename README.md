# hookd 
The primary use case of hookd is to use it inside a deployment pipeline and execute specific tasks on a remote server during a job.

An example config will be generated if hookd can't find one.
```toml
[[hooks]]
id = 'blog'
secret = 'b0b4d001-da79-4db8-bcaa-3130774abcea'
execute = 'start cmd.exe'

[[hooks]]
id = 'website'
secret = 'af322f5c-4586-4ad8-80bc-23e132e85f9f'
execute = 'sleep 1'
```

`id` is not used by hookd and helps to identify what the hook does.

`secret` will be used as a path parameter to secure the hook.

`execute` is the command to execute.

You can invoke a webhook using curl: `curl server.com/af322f5c-4586-4ad8-80bc-23e132e85f9f`
