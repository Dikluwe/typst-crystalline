
impl SealedPositions {
    pub fn lab_position_count_on_or_after(&self, page: usize) -> usize {
        self.positions.values().filter(|position| position.page.get() >= page).count()
    }


    pub fn lab_position_overlay(&self, overlay: &[(Location, Position)]) -> Option<Self> {
        let mut replacements = HashMap::with_capacity(overlay.len());
        for &(location, value) in overlay {
            let witness = self.positions.get(&location)?;
            let x = value.point.x.val();
            let y = value.point.y.val();
            if value.page != witness.page
                || !x.is_finite()
                || !y.is_finite()
                || x.to_bits() != witness.point.x.val().to_bits()
                || y.to_bits() != witness.point.y.val().to_bits()
                || replacements.insert(location, value).is_some()
            {
                return None;
            }
        }
        let mut positions = self.positions.clone();
        positions.extend(replacements);
        Some(Self::from_runtime(positions))
    }
}
