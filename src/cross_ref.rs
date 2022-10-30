use crate::ops;

pub fn cross_reference(
    prg: &mut Vec<ops::Instruction>,
) -> Result<Vec<ops::Instruction>, (&'static str, ops::Pos)> {
    let mut stack: Vec<usize> = vec![];
    for i in 0..prg.len() {
        let token = prg[i].op.clone();
        match token {
            ops::Operator::If=> stack.push(i),
            ops::Operator::While => stack.push(i),
            ops::Operator::Else => {
                let if_i = stack.pop().unwrap();
                if prg[if_i].op == ops::Operator::If {
                    prg[if_i].arg = Some(i);
                    stack.push(i)
                } else {
                    return Err(("'ante-la' can only close 'la' blocks", prg[if_i].pos.clone()));
                }
            }
            ops::Operator::End => {
                let block_i = stack.pop().unwrap();

                if prg[block_i].op == ops::Operator::If {
                    prg[block_i].arg = Some(i);
                } else if prg[block_i].op == ops::Operator::Else {
                    prg[block_i].arg = Some(i);
                } else if prg[block_i].op == ops::Operator::Do {
                    prg[i].arg = prg[block_i].arg;
                    prg[block_i].arg = Some(i);
                }
            }
            ops::Operator::Do => {
                let while_i = stack.pop().unwrap();
                prg[i].arg = Some(while_i);
                stack.push(i)
            }
            _ => (),
        }
    }
    if stack.len() > 0 {
        return Err(("Unclosed block", prg[stack.pop().unwrap()].pos.clone()));
    }

    Ok(prg.clone())
}
