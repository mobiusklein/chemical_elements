use crate::table::{populate_periodic_table, PERIODIC_TABLE};
use crate::{
    ChemicalComposition, ElementSpecification, ElementSpecificationParsingError,
    FormulaParserError, PeriodicTable,
};

#[allow(non_snake_case)]
/** A helper data structure that encapsulates
a [`PeriodicTable`], along with some pre-parsed
[`ElementSpecification`] and [`ChemicalComposition`]
instances for convenience.
*/
pub struct ChemicalElements<'lifespan> {
    pub periodic_table: PeriodicTable,
    pub C: ElementSpecification<'lifespan>,
    pub H: ElementSpecification<'lifespan>,
    pub O: ElementSpecification<'lifespan>,
    pub N: ElementSpecification<'lifespan>,
    pub S: ElementSpecification<'lifespan>,
    pub H2O: ChemicalComposition<'lifespan>,
    pub OH: ChemicalComposition<'lifespan>,
    pub NH2: ChemicalComposition<'lifespan>,
}

impl<'transient, 'lifespan: 'transient> ChemicalElements<'lifespan> {
    fn make_periodic_table() -> PeriodicTable {
        let mut periodic_table = PeriodicTable::new();
        populate_periodic_table(&mut periodic_table);
        periodic_table
    }

    pub fn new() -> ChemicalElements<'lifespan> {
        let periodic_table = Self::make_periodic_table();

        let ce = ChemicalElements {
            periodic_table,
            C: ElementSpecification::parse_with("C", &PERIODIC_TABLE).unwrap(),
            H: ElementSpecification::parse_with("H", &PERIODIC_TABLE).unwrap(),
            O: ElementSpecification::parse_with("O", &PERIODIC_TABLE).unwrap(),
            N: ElementSpecification::parse_with("N", &PERIODIC_TABLE).unwrap(),
            S: ElementSpecification::parse_with("S", &PERIODIC_TABLE).unwrap(),
            H2O: ChemicalComposition::parse_with("H2O", &PERIODIC_TABLE).unwrap(),
            OH: ChemicalComposition::parse("OH").unwrap(),
            NH2: ChemicalComposition::parse_with("NH2", &PERIODIC_TABLE).unwrap(),
        };
        ce
    }

    #[inline]
    pub fn parse_formula(
        &self,
        string: &'transient str,
    ) -> Result<ChemicalComposition, FormulaParserError> {
        ChemicalComposition::parse_with(string, &self.periodic_table)
    }

    #[inline]
    pub fn parse_element(
        &self,
        string: &'transient str,
    ) -> Result<ElementSpecification, ElementSpecificationParsingError> {
        ElementSpecification::parse_with(string, &self.periodic_table)
    }
}

impl Default for ChemicalElements<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ce() {
        let ce = ChemicalElements::new();
        let c_elt = ce.parse_element("C").unwrap();
        assert_eq!(ce.C, c_elt);
    }
}
