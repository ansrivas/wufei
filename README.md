# WUFEI
Wufei is an async Rust CLI Tool for the aggregation of Kubernetes logs. This tool will write kubernetes logs for each pod down to a container level to a file or to stdout depending on the developers needs and also has the ability to log new pods that are spun up in the namespace as well. There is an informer written to let Wufei know when new pods spin up!

Heavily inspired by https://github.com/johanhaleby/kubetail Kubetail.


![Wufei](wufei.jpeg?raw=true "Wufei")

## Installation
As of right now, Wufei is NOT part of cargo.  Its on my todo list.  Right now just do cargo build in the root of the the project, and then access the wufei in target/debug/wufei
```bash
cargo run -- --namespace=<my-kube-namespace> --color
```

## Example Output
Video coming soon

## CLI Arguments
```
Wufei 0.2.1
Eric McBride <ericmcbridedeveloper@gmail.com> github.com/ericmcbride
Tail ALL your kubernetes logs at once, or record them to files

USAGE:
    wufei [FLAGS] [OPTIONS]

FLAGS:
        --color      Pods for the logs will appear in color in your terminal
    -f, --file       Record the logs to a file. Note: Logs will not appear in stdout
    -h, --help       Prints help information
        --update     Runs an informer, that will add new pods to the tailed logs
    -V, --version    Prints version information

OPTIONS:
    -n, --namespace <namespace>    Namespace for logs [default: kube-system]
    -o, --outfile <outfile>        Outfile of where the logs are being recorded [default: /tmp/wufei/]
```

Wufei requires a namespace.
- The color flog `--color` will display pod names in colors in stdout.
- The file flag `--file` will write the logs to /tmp/wufei/<podname> based on pod name.
- The update flag `--update` will spin up an informer that will listen for new pods to spin up

- The namespace option `--namespace` is the namespace the developer wants to use to tail logs from
- The outfile option `--outfile` is used when the file flag is used, to change the location of
  where the files are used

Example:

```
cargo run -- --namespace=dev --color
```

## WUFEI USES YOUR CURRENT KUBE CONTEXT
$KUBECONFIG=:$KUBECONFIG/path/to/your/config

#### LIST CONTEXTS
`kubectl config view`

#### USE CONTEXT
`kubectl config use-context my-context`

## TOO MANY OPEN FILES ERROR
This error will pop up, depending on the settings on your operating system.  This is due to
security reasons.  Below is how you would fix this on a Mac.
#### OS/X
`ulimit -n 2048`

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
