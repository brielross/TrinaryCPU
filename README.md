This project is a trinary cpu which uses a three state system instead of binary's two states. This gives the possible value of each "trip" as 0, 1, or 2.

# Running the cpu
<ol>
  <li>Currently, the cpu only lets you run one instruction at a time by modifying the <code>curr_instruction</code> variable on line 50 of <code>main.rs</code>.</li>
  <li>Run <code>rustc main.rs</code></li>
  <li>Execute resulting main binary</li>
</ol>

# Registers and memory
## Registers
There are 27 spaces in the register, and there are two registers that are set by default with <code>R[0] = 0</code> and <code>R[1] = 1</code>. These two should be held constant to derive other numbers.
## Memory
Memory has 2187 spaces

# How instructions work
Each instruction is 12 trips long and starts with two trips for the operation to perform.
The main downside of this insturction set is that all numbers have to be derived by adding from 1 using `R[1]`. 
- Register type operations
  - Trip routing
    - 11-10 are instruction
    - 9-7 are source register 1
    - 6-4 are source register 2
    - 3-1 are result register
    - 0 is unused
  - Operations
    - Add (00) (Result register = source 1 + source 2)
    - Sub (01) (Result register = source 1 - source 2)
    - Boz (02) (If source 2 == 0, jump to address in source 1)
    - Slt (10) (Source 1 < source 2 ?: result = 2 else result = 0)
    - Jr (12) (Jump to the address stored in source 1)
- Immediate type operations
  - Trip routing
    - 11-10 are instruction
    - 9-7 are the register
    - 6-0 is the immediate
  - Operations
    - Sw (20) (Store register value to memory at address of the immediate)
    - Lw (21) (Store contents from the memory address of the immediate to the register)
- Jump type operations
  - Trip routing
    - 11-10 are instruction
    - 9-0 are the address
  - Operations
    - J (11) (jump PC to address of immediate)

## Examples: 
- 000210021001 (00 021 002 100 1) (`R[7] + R[2]` gets stored into `R[9]`)
- 012000102220 (01 200 010 221 0) (`R[18] – R[3]` gets stored into `R[25]`)
- 021000100011 (02 100 010 001 1) (if `R[3] == 0`, jump to instruction in `R[9]`)
- 101000100011 (10 100 010 001 1) (if `R[9] < R[3]`, `R[1] = 2` else `R[1] = 0`)
- 110000001000 (11 0000001000) (jump to address 27)
- 121000000001 (12 100 000 000 1) (jump to address stored in `R[9]`)
- 201000102101 (20 100 0102101) (store `R[9]` to `M[309]`)
- 211000102101 (21 100 0102101) (load `M[309]` to `R[9]`)

# Future work
<ul>
  <li>Read instructions from file and execute sequentially</li>
  <li>Make instruction 22 a nop</li>
</ul>
