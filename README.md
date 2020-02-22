# kubectl-spy

kubectl plugin for reading values from secrets

This project is meant as a teaching aid for a presentation about kubernetes client libraries,
I would *not* recommend using this project in a real world scenario.

For a more feature complete tool I would probably use [kubectl-view-secret](https://github.com/elsesiy/kubectl-view-secret),
which can be installed using [krew](https://github.com/kubernetes-sigs/krew).

## Requirements

* linux or mac (BSD or windows might work, but I haven't tested it)
* kubernetes cluster (I am using kind on linux and docker desktop on mac)
* kubectl setup and connected to your cluster
* node and npm for building

## Building

Building is handled by a makefile, by simply running make you can compile a binary.

```shell script
make
````

This will output the binary to the `bin` folder, so for development testing you can run

```shell script
./bin/kubectl-spy my-secret-name
```

## Usage

In order to install the binary, you can use the install make target, or simply copy the binary to anywhere within your `$PATH`.

```shell script
make install
```

```shell script
cp ./bin/kubectl-spy /some/custom/path/folder
```

Once it's within your `$PATH` kubectl will automatically find it as a plugin and you can use it with

```shell script
kubectl spy my-secret-name
```
