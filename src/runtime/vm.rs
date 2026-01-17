use super::env::Env;

pub enum OpCode {
    Load(String),
    T,
    F,
    And,
    Or,
    Not,
    Used,
}

impl Default for OpCode {
    fn default() -> Self {
        OpCode::Used
    }
}

pub struct VM<'a> {
    env: &'a mut Env,
    values: Vec<bool>,
    instructions: Vec<OpCode>,
    cur: usize,
}

impl<'a> VM<'a> {
    pub fn new(env: &'a mut Env, instructions: Vec<OpCode>) -> VM<'a> {
        VM { env, values: Vec::new(), instructions, cur: 0 }
    }

    fn load(&mut self, name: &String) -> Result<(), String> {
        let val = self.env.get(name).ok_or(
            format!("Undefined variable '{}'", name)
        )?;
        self.values.push(val);
        Ok(())
    }

    fn execute_next(&mut self, cur: usize) -> Result<(), String> {
        macro_rules! command {
            ($op:tt) => {
                {
                    let a = self.values.pop().unwrap();
                    let b = self.values.pop().unwrap();
                    self.values.push(a $op b)
                }
            }
        }

        use OpCode::*;
        match std::mem::take(&mut self.instructions[cur]) {
            Load(name) => self.load(&name)?,
            T => self.values.push(true),
            F => self.values.push(false),
            And => command!(&&),
            Or => command!(||),
            Not => {
                let a = self.values.pop().unwrap();
                self.values.push(!a);
            },
            Used => unreachable!()
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<bool, String> {
        while self.cur < self.instructions.len() {
            self.execute_next(self.cur)?;
            self.cur += 1;
        }
        Ok(self.values.pop().unwrap())
    }
}
