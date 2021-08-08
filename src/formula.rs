use std::num::ParseIntError;
use crate::ChemicalComposition;
use crate::ElementSpecification;
use crate::{Element, PeriodicTable};
use crate::table::PERIODIC_TABLE;


#[derive(Debug)]
pub enum FormulaParserState {
    New,
    Element,
    Isotope,
    IsotopeToCount,
    Count,
    Group,
    GroupToGroupCount,
    GroupCount
}

impl Default for FormulaParserState {
    fn default() -> FormulaParserState {
        FormulaParserState::New
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FormulaParserError {
    InvalidStart,
    ElementCountMalformed,
    IsotopeCountMalformed,
    GroupCountMalformed,
    IncompleteFormula,
    InvalidElement,
}


pub fn parse_formula(string: &str) -> Result<ChemicalComposition, FormulaParserError> {
    parse_formula_with_table(string, &PERIODIC_TABLE)
}


#[derive(Default)]
pub struct FormulaParser {
    pub element_start: usize,
    pub element_end: usize,
    pub isotope_start: usize,
    pub isotope_end: usize,
    pub count_start: usize,
    pub count_end: usize,
    pub paren_stack: i32,
    pub group_start: usize,
    pub group_end: usize,
    pub group_count_start: usize,
    pub group_count_end: usize,
    pub state: FormulaParserState,
}


impl<'transient, 'lifespan:'transient, 'separate> FormulaParser {

    pub fn parse(string: &str) -> Result<ChemicalComposition<'lifespan>, FormulaParserError> {
        let mut parser = Self::default();
        parser.parse_formula_with_table(string, &PERIODIC_TABLE)
    }

    pub fn parse_with_table(string: &str, periodic_table: &'lifespan PeriodicTable) -> Result<ChemicalComposition<'lifespan>, FormulaParserError> {
        let mut parser = Self::default();
        parser.parse_formula_with_table(string, periodic_table)
    }

    pub fn parse_element_from_string(&mut self, string: &str, periodic_table: &'lifespan PeriodicTable) -> &'lifespan Element {
        let elt_sym = &string[self.element_start..self.element_end];
        let elt = &periodic_table[elt_sym];
        self.element_start = 0;
        self.element_end = 0;
        elt
    }

    pub fn parse_element_count(&mut self, string: &str) -> Result<i32, ParseIntError> {
        let count_parse = string[self.count_start..self.count_end].parse::<i32>();
        self.count_start = 0;
        self.count_end = 0;
        count_parse
    }

    pub fn parse_group_count(&mut self, string: &str) -> Result<i32, ParseIntError> {
        let count_parse = string[self.group_count_start..self.group_count_end].parse::<i32>();
        self.group_count_start = 0;
        self.group_count_end = 0;
        count_parse
    }

    pub fn parse_formula_with_table(&mut self, string: &str, periodic_table: &'lifespan PeriodicTable) -> Result<ChemicalComposition<'lifespan>, FormulaParserError> {
        let mut acc = ChemicalComposition::new();
        let n = string.len();

        for (i, c) in string.char_indices() {
            match self.state {
                FormulaParserState::New => {
                    if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                        self.element_start = i;
                        self.state = FormulaParserState::Element;
                    } else if c == '(' {
                        self.paren_stack += 1;
                        self.group_start = i + 1;
                        self.state = FormulaParserState::Group;
                    } else {
                        return Err(FormulaParserError::InvalidStart);
                    }
                },
                FormulaParserState::Group => {
                    if c == ')' {
                        self.paren_stack -= 1;
                        if self.paren_stack == 0 {
                            self.group_end = i;
                            self.state = FormulaParserState::GroupToGroupCount;
                        }
                    } else if c == '(' {
                        self.paren_stack += 1;
                    }
                },
                FormulaParserState::Element => {
                    if c.is_ascii_alphabetic() {
                        if c.is_uppercase() {
                            self.element_end = i;
                            let elt = self.parse_element_from_string(string, periodic_table);
                            let elt_spec = ElementSpecification {
                                element: elt,
                                isotope: 0
                            };
                            acc.inc(elt_spec, 1);
                            self.state = FormulaParserState::Element;
                            self.element_start = i;
                            self.element_end = 0;
                        }
                    } else if c.is_numeric() {
                        self.element_end = i;
                        self.count_start = i;
                        self.state = FormulaParserState::Count;
                    } else if c == '[' {
                        self.isotope_start = i + 1;
                        self.state = FormulaParserState::Isotope;
                    } else if c == '(' {
                        self.element_end = i;
                        let elt = self.parse_element_from_string(string, periodic_table);
                        let elt_spec = ElementSpecification {
                            element: elt,
                            isotope: 0
                        };
                        acc.inc(elt_spec, 1);

                        self.paren_stack += 1;
                        self.group_start = i + 1;
                        self.state = FormulaParserState::Group;

                    }
                },
                FormulaParserState::Isotope => {
                    if c == ']' {
                        self.isotope_end = i;
                        self.state = FormulaParserState::IsotopeToCount;
                    } else if !c.is_numeric() {
                        return Err(FormulaParserError::IsotopeCountMalformed)
                    }
                },
                FormulaParserState::Count => {
                    if !c.is_numeric() {
                        self.count_end = i;
                        let count_parse = self.parse_element_count(string);
                        let count: i32 = match count_parse {
                            Ok(val) => {val},
                            Err(_msg) => {return Err(FormulaParserError::ElementCountMalformed);}
                        };
                        let isotope: u16 = if self.isotope_end != self.isotope_start {
                            match string[self.isotope_start..self.isotope_end].parse::<u16>() {
                                Ok(val) => {val},
                                Err(_msg) => {return Err(FormulaParserError::IsotopeCountMalformed);}
                            }
                        } else {
                            0
                        };

                        let elt = self.parse_element_from_string(string, periodic_table);
                        let elt_spec = ElementSpecification {
                            element: elt,
                            isotope: isotope
                        };
                        acc.inc(elt_spec, count);
                        self.isotope_start = 0;
                        self.isotope_end = 0;
                        self.count_start = 0;
                        self.count_end = 0;
                        if c == '(' {
                            self.paren_stack = 1;
                            self.group_start = i + 1;
                            self.state = FormulaParserState::Group;
                        } else if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                            self.element_start = i;
                            self.state = FormulaParserState::Element;
                        } else {
                            return Err(FormulaParserError::InvalidElement);
                        }
                    }
                },
                FormulaParserState::IsotopeToCount => {
                    if c.is_numeric() {
                        self.count_start = i;
                        self.state = FormulaParserState::Count;
                    } else {
                        let elt = self.parse_element_from_string(string, periodic_table);
                        let isotope: u16 = match string[self.isotope_start..self.isotope_end].parse::<u16>() {
                            Ok(val) => {val},
                            Err(_msg) => {return Err(FormulaParserError::IsotopeCountMalformed);}
                        };
                        let elt_spec = ElementSpecification {
                            element: elt,
                            isotope: isotope
                        };
                        acc.inc(elt_spec, 1);
                        self.isotope_start = 0;
                        self.isotope_end = 0;

                        if c == '(' {
                            self.paren_stack += 1;
                            self.group_start = i + 1;
                            self.state = FormulaParserState::Group;
                        } else if c.is_ascii_uppercase() {
                            self.element_start = i;
                            self.state = FormulaParserState::Element;
                        } else {
                            return Err(FormulaParserError::IsotopeCountMalformed);
                        }
                    }
                },
                FormulaParserState::GroupToGroupCount => {
                    if !c.is_numeric() {
                        let group = match Self::parse_with_table(&string[self.group_start..self.group_end], periodic_table) {
                            Ok(grp) => {grp},
                            Err(err) => {return Err(err)}
                        };
                        self.group_start = 0;
                        self.group_end = 0;
                        acc += &group;
                        if c == '(' {
                            self.paren_stack = 1;
                            self.group_start = i + 1;
                            self.state = FormulaParserState::Group;
                        } else if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                            self.element_start = i;
                            self.state = FormulaParserState::Element;
                        } else {
                            return Err(FormulaParserError::InvalidElement);
                        }
                    } else {
                        self.group_count_start = i;
                        self.state = FormulaParserState::GroupCount;
                    }
                }
                FormulaParserState::GroupCount => {
                    if !c.is_numeric() {
                        self.group_count_end = i;
                        let group = match Self::parse_with_table(&string[self.group_start..self.group_end], periodic_table) {
                            Ok(grp) => {grp},
                            Err(err) => {return Err(err)}
                        };
                        self.group_start = 0;
                        self.group_end = 0;

                        let group_count: i32 = match self.parse_group_count(string) {
                            Ok(val) => {val},
                            Err(_msg) => {return Err(FormulaParserError::ElementCountMalformed);}
                        };
                        acc += &(&group * group_count);

                        if c == '(' {
                            self.paren_stack = 1;
                            self.group_start = i + 1;
                            self.state = FormulaParserState::Group;
                        } else if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                            self.element_start = i;
                            self.state = FormulaParserState::Element;
                        } else {
                            return Err(FormulaParserError::InvalidElement);
                        }
                    }
                }
            }
        }

        let i = n;
        match self.state {
            FormulaParserState::Element => {
                self.element_end = i;
                let elt = self.parse_element_from_string(string, periodic_table);
                let elt_spec = ElementSpecification {
                    element: elt,
                    isotope: 0
                };
                acc.inc(elt_spec, 1);
            },
            FormulaParserState::Count => {
                self.count_end = i;
                let count: i32 = match self.parse_element_count(string) {
                    Ok(val) => {val},
                    Err(_msg) => {return Err(FormulaParserError::ElementCountMalformed);}
                };
                let isotope: u16 = if self.isotope_end != self.isotope_start {
                    match string[self.isotope_start..self.isotope_end].parse::<u16>() {
                        Ok(val) => {val},
                        Err(_msg) => {return Err(FormulaParserError::IsotopeCountMalformed);}
                    }
                } else {
                    0
                };
                let elt = self.parse_element_from_string(string, periodic_table);
                let elt_spec = ElementSpecification {
                    element: elt,
                    isotope: isotope
                };
                acc.inc(elt_spec, count);
            },
            FormulaParserState::GroupToGroupCount => {
                let group = match Self::parse_with_table(&string[self.group_start..self.group_end], periodic_table) {
                    Ok(grp) => {grp},
                    Err(err) => {return Err(err)}
                };
                acc += &group;
            }
            FormulaParserState::GroupCount => {
                self.group_count_end = i;
                let group = match Self::parse_with_table(&string[self.group_start..self.group_end], periodic_table) {
                    Ok(grp) => {grp},
                    Err(err) => {return Err(err)}
                };
                self.group_start = 0;
                self.group_end = 0;


                let group_count: i32 = match self.parse_group_count(string) {
                    Ok(val) => {val},
                    Err(_msg) => {return Err(FormulaParserError::GroupCountMalformed);}
                };
                acc += &(&group * group_count);
            },
            _ => {
                return Err(FormulaParserError::IncompleteFormula)
            }
        }
        return Ok(acc);
    }
}


pub fn parse_formula_with_table<'lifespan>(string: &str, periodic_table: &'lifespan PeriodicTable) -> Result<ChemicalComposition<'lifespan>, FormulaParserError> {
    FormulaParser::parse_with_table(string, periodic_table)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_obj() {
        let res = FormulaParser::parse("H2O").unwrap();
        let hydrogen = ElementSpecification::parse("H").unwrap();
        let oxygen = ElementSpecification::parse("O").unwrap();
        assert_eq!(res[&hydrogen], 2);
        assert_eq!(res[&oxygen], 1);
    }
}