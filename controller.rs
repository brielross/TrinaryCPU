/*
WriteReg, MemReadWrite, RegWrite, RegWriteData, PCSrc, WriteRegSrc
Add: 2002000
Sub: 2102000
Boz: 00000(0/2)0
Slt: 2202000
J: 0000010
Jr: 0000020
Sw: 0020000
Lw: 2011200
Jal: 2001112
*/

pub struct ControlCode {
    pub write_reg: char,         // 0 - don't write, 2 - write
    pub alu_code: char,          // 0 - add, 1 - sub, 2 - check less than
    pub mem_read_write: char,    // 0 - nothing, 1 - read, 2 - write
    pub reg_write: char,         // 0 - don't write, 2 - write
    pub reg_write_data: char,    // 0 - use alu result, 1 - use currPC, 2 - use mem result
    pub pc_src: char,            // 0 - PC + 1, 1 - trips 9-0, 2 - data 1 from register
    pub write_reg_scr: char      // 0 - trips 3-1, 2 - "111"
}

pub fn get_control_codes(op_code: String, read_one_is_zero: bool) -> ControlCode {
    return match op_code.as_str() {
        "00" => { // Add
            ControlCode {
                write_reg: '2',
                alu_code: '0',
                mem_read_write: '0',
                reg_write: '2',
                reg_write_data: '0',
                pc_src: '0',
                write_reg_scr: '0'
            }

        },
        "01" => { // Sub
            ControlCode {
                write_reg: '2',
                alu_code: '1',
                mem_read_write: '0',
                reg_write: '2',
                reg_write_data: '0',
                pc_src: '0',
                write_reg_scr: '0'
            }
        },
        "02" => { // BOZ
            if read_one_is_zero {
                ControlCode {
                    write_reg: '0',
                    alu_code: '0',
                    mem_read_write: '0',
                    reg_write: '0',
                    reg_write_data: '0',
                    pc_src: '2',
                    write_reg_scr: '0'
                }
            } else {
                ControlCode {
                    write_reg: '0',
                    alu_code: '0',
                    mem_read_write: '0',
                    reg_write: '0',
                    reg_write_data: '0',
                    pc_src: '0',
                    write_reg_scr: '0'
                }
            }  
        },
        "10" => { // SLT
            ControlCode {
                write_reg: '2',
                alu_code: '2',
                mem_read_write: '0',
                reg_write: '2',
                reg_write_data: '0',
                pc_src: '0',
                write_reg_scr: '0'
            }
        },
        "11" => { // J
            ControlCode {
                write_reg: '0',
                alu_code: '0',
                mem_read_write: '0',
                reg_write: '0',
                reg_write_data: '0',
                pc_src: '1',
                write_reg_scr: '0'
            }
        },
        "12" => { // JR
            ControlCode {
                write_reg: '0',
                alu_code: '0',
                mem_read_write: '0',
                reg_write: '0',
                reg_write_data: '0',
                pc_src: '2',
                write_reg_scr: '0'
            }
        },
        "20" => { // SW
            ControlCode {
                write_reg: '0',
                alu_code: '0',
                mem_read_write: '2',
                reg_write: '0',
                reg_write_data: '0',
                pc_src: '0',
                write_reg_scr: '0'
            }
        },
        "21" => { // LW
            ControlCode {
                write_reg: '2',
                alu_code: '0',
                mem_read_write: '1',
                reg_write: '2',
                reg_write_data: '2',
                pc_src: '0',
                write_reg_scr: '0'
            }
        },
        _ => { // Unreachable
            ControlCode {
                write_reg: '2',
                alu_code: '0',
                mem_read_write: '0',
                reg_write: '1',
                reg_write_data: '1',
                pc_src: '1',
                write_reg_scr: '2'
            }
        }
    }
}