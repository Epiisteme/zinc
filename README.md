# The Zinc framework

The goal of Zinc is to make writing safe zero-knowledge programs and ZKP-based
smart contracts easy.

## Workflow example

At first, you should install the following binaries into your `PATH`:
- `zargo` - the circuit management tool
- `znc` - the Zinc compiler
- `zvm` - the Zinc virtual machine
- `schnorr` - the Schnorr signature tool (optional)

Then, follow the example to create and use your first circuit:

```bash
# create a new circuit called 'zircuit'
zargo new zircuit
cd zircuit/

# write some code in the circuit

# build the circuit
zargo build

# fill the witness input JSON usually located at ./data/witness.json with values

# runs the circuit to check it without input data
zargo run

# generate the prover parameters
zargo setup

# generate the proof
zargo prove > './data/proof.txt'

# verify the proof
zargo verify < './data/proof.txt'
```

**OR**

```bash
# create a new circuit called 'zircuit'
zargo new zircuit
cd zircuit/

# write some code in the circuit

# build & run & setup & prove & verify
zargo proof-check

# fill the witness input JSON usually located at ./data/witness.json with values

# repeat the sequence
zargo proof-check
```
