use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct BirthYear(u16);
impl FromStr for BirthYear {
    type Err = ();

    fn from_str(s: &str) -> Result<BirthYear, ()> {
        let year = s.parse().map_err(|_| ())?;
        if year < 1920 || year > 2002 {
            return Err(());
        }
        Ok(BirthYear(year))
    }
}

#[derive(Debug)]
struct IssueYear(u16);
impl FromStr for IssueYear {
    type Err = ();

    fn from_str(s: &str) -> Result<IssueYear, ()> {
        let year = s.parse().map_err(|_| ())?;
        if year < 2010 || year > 2020 {
            return Err(());
        }
        Ok(IssueYear(year))
    }
}

#[derive(Debug)]
struct ExpYear(u16);
impl FromStr for ExpYear {
    type Err = ();

    fn from_str(s: &str) -> Result<ExpYear, ()> {
        let year = s.parse().map_err(|_| ())?;
        if year < 2020 || year > 2030 {
            return Err(());
        }
        Ok(ExpYear(year))
    }
}

#[derive(Debug)]
struct HairColor(u32);
impl FromStr for HairColor {
    type Err = ();

    fn from_str(s: &str) -> Result<HairColor, ()> {
        let hex = s.strip_prefix("#").ok_or(())?;
        let hcl = u32::from_str_radix(hex, 16).map_err(|_| ())?;
        if hcl > 0xffffff {
            return Err(());
        }
        Ok(HairColor(hcl))
    }
}

#[derive(Debug)]
enum Height {
    Inches(u8),
    Cm(u8),
}
impl FromStr for Height {
    type Err = ();

    fn from_str(s: &str) -> Result<Height, ()> {
        if let Some(cm) = s.strip_suffix("cm") {
            let cm: u8 = cm.parse().map_err(|_| ())?;
            if cm < 150 || cm > 193 {
                return Err(());
            }
            Ok(Height::Cm(cm))
        } else if let Some(inches) = s.strip_suffix("in") {
            let inches: u8 = inches.parse().map_err(|_| ())?;
            if inches < 59 || inches > 76 {
                return Err(());
            }
            Ok(Height::Inches(inches))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}
impl FromStr for EyeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<EyeColor, ()> {
        Ok(match s {
            "amb" => EyeColor::Amber,
            "blu" => EyeColor::Blue,
            "brn" => EyeColor::Brown,
            "gry" => EyeColor::Gray,
            "grn" => EyeColor::Green,
            "hzl" => EyeColor::Hazel,
            "oth" => EyeColor::Other,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
struct PassportId(u32);
impl FromStr for PassportId {
    type Err = ();

    fn from_str(s: &str) -> Result<PassportId, ()> {
        if s.len() != 9 {
            return Err(());
        }
        Ok(PassportId(s.parse().map_err(|_| ())?))
    }
}

#[derive(Debug)]
pub struct Passport<'a> {
    byr: BirthYear,
    iyr: IssueYear,
    eyr: ExpYear,
    hgt: Height,
    hcl: HairColor,
    ecl: EyeColor,
    pid: PassportId,
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        fn parse_kv(input: &str) -> Option<(&str, &str)> {
            let mut iter = input.split(':');
            let k = iter.next()?;
            let v = iter.next()?;
            if iter.next().is_some() {
                return None;
            }
            Some((k, v))
        }

        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;
        for field in input.split_ascii_whitespace() {
            match parse_kv(field) {
                Some(("byr", value)) => byr = Some(value.parse().ok()?),
                Some(("iyr", value)) => iyr = Some(value.parse().ok()?),
                Some(("eyr", value)) => eyr = Some(value.parse().ok()?),
                Some(("hgt", value)) => hgt = Some(value.parse().ok()?),
                Some(("hcl", value)) => hcl = Some(value.parse().ok()?),
                Some(("ecl", value)) => ecl = Some(value.parse().ok()?),
                Some(("pid", value)) => pid = Some(value.parse().ok()?),
                Some(("cid", value)) => cid = Some(value),
                _ => return None,
            }
        }
        Some(Passport {
            byr: byr?,
            iyr: iyr?,
            eyr: eyr?,
            hgt: hgt?,
            hcl: hcl?,
            ecl: ecl?,
            pid: pid?,
            cid,
        })
    }
}

#[aoc(day4, part2)]
pub fn solve_d4_p2(input: &str) -> usize {
    input.split("\n\n").filter_map(Passport::parse).count()
}
