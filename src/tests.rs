use super::*;

#[cfg(test)]

#[test]
//#[ignore]
fn date_06121973() {
    let date: Date = Date::new(6, 12, 1973);
    assert_eq!(date.leap_years(), Some(1));
}

#[test]
//#[ignore]
fn date_01011970() {
    let date: Date = Date::new(1, 1, 1970);
    assert_eq!(date.leap_years(), Some(0));
}

#[test]
//#[ignore]
fn date_02031980() {
    let date: Date = Date::new(2, 3, 1980);
    assert_eq!(date.leap_years(), Some(2));
}

#[test]
//#[ignore]
fn date_130290249600() {
    assert_eq!(Date::date_timestamp(130290249600), Date::new(25, 9, 6098
    ));
}

#[test]
//#[ignore]
fn date_1585872000() {
    assert_eq!(Date::date_timestamp(1585872000), Date::new(3, 4, 2020));
}

#[test]
//#[ignore]
fn date_0() {
    assert_eq!(Date::date_timestamp(0), Date::new(1, 1, 1970));
}

#[test]
//#[ignore]
fn date_81043200() {
    assert_eq!(Date::date_timestamp(81043200), Date::new(27, 7, 1972));
}

#[test]
//#[ignore]
fn date_28620172800() {
    assert_eq!(Date::date_timestamp(28620172800), Date::new(8, 12, 2876));
}



    #[test]
    //#[ignore]
    fn day_20200403() {
        let date: Date = Date::new(3, 4, 2020);
        assert_eq!(date.timestamp(), Some(1585872000));
    }

    #[test]
    //#[ignore]
    fn day_19690202() {
        let date: Date = Date::new(2, 2, 1969);
        assert_eq!(date.timestamp(), None);
    }

    #[test]
    //#[ignore]
    fn day_20411117() {
        let date: Date = Date::new(17, 11, 2041);
        assert_eq!(date.timestamp(), Some(2268259200));
    }

    #[test]
    //#[ignore]
    fn day_19840101() {
        let date: Date = Date::new(1, 1, 1984);
        assert_eq!(date.timestamp(), Some(441763200));
    }

    #[test]
    //#[ignore]
    fn day_19701007() {
        let date: Date = Date::new(7, 10, 1970);
        assert_eq!(date.timestamp(), Some(24105600));
    }
