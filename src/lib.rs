pub mod hsh_cmds;

#[cfg(test)]
mod tests {
    use super::hsh_cmds::*;

    #[test]
    fn display_flag_vec() {
        let f1 = Flag::new('a');
        let f2 = Flag::new('b');
        let f3 = Flag::new('c');
        let f4 = Flag::new('d');
        let f5 = Flag::new('e');

        let flags = Flags(vec![f1, f2, f3, f4, f5]);
        
        let f6 = Flag::new('e');
        assert!(flags.contains(&f6));
        assert_eq!(format!("{}", flags), format!("-abcde"))
    }

    #[test]
    fn doc_test() {
        // let mut flag_l: Flag;
        // if let Ok(f) = "-l".parse::<Flag>() {
        //     flag_l = f;
        // } else {
        //     flag_l = Flag::new('l');
        // }
        let flag_l = "l".parse::<Flag>().unwrap();
        assert_eq!(flag_l, Flag('l'));
        // assert_eq!(format!("{}", flag_l), format!("{}", Flag('k')));
        assert_eq!(format!("{}", flag_l), format!("l"))
    }

}
