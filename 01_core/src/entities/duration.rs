//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/duration.md
//! @prompt-hash a08eb4f7
//! @layer L1
//! @updated 2026-06-22
//!
//! Intervalo de tempo — `Value::Duration`.
//! Passo 400: tipo L1 puro, zero I/O; representação interna em nanossegundos.

const NANOS_PER_SECOND: u64 = 1_000_000_000;

/// Intervalo de tempo (não ponto no tempo).
///
/// Representado internamente por um `u64` de nanossegundos — `Copy`,
/// puro-Rust e sem alocação dinâmica.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    pub nanos: u64,
}

impl Duration {
    /// Duração zero.
    pub const ZERO: Self = Self { nanos: 0 };

    /// Um segundo.
    pub const SECOND: Self = Self { nanos: NANOS_PER_SECOND };

    /// Um minuto.
    pub const MINUTE: Self = Self { nanos: 60 * NANOS_PER_SECOND };

    /// Uma hora.
    pub const HOUR: Self = Self { nanos: 3_600 * NANOS_PER_SECOND };

    /// Um dia.
    pub const DAY: Self = Self { nanos: 86_400 * NANOS_PER_SECOND };

    /// Cria uma duração a partir de nanossegundos.
    pub fn from_nanos(nanos: u64) -> Self {
        Self { nanos }
    }

    /// Cria uma duração a partir de segundos.
    pub fn from_seconds(seconds: u64) -> Self {
        Self {
            nanos: seconds * NANOS_PER_SECOND,
        }
    }

    /// Cria uma duração a partir de minutos.
    pub fn from_minutes(minutes: u64) -> Self {
        Self {
            nanos: minutes * Self::MINUTE.nanos,
        }
    }

    /// Cria uma duração a partir de horas.
    pub fn from_hours(hours: u64) -> Self {
        Self {
            nanos: hours * Self::HOUR.nanos,
        }
    }

    /// Cria uma duração a partir de dias.
    pub fn from_days(days: u64) -> Self {
        Self {
            nanos: days * Self::DAY.nanos,
        }
    }

    /// Total de segundos inteiros (truncado).
    pub fn as_seconds(&self) -> u64 {
        self.nanos / NANOS_PER_SECOND
    }

    /// Total de minutos inteiros (truncado).
    pub fn as_minutes(&self) -> u64 {
        self.nanos / Self::MINUTE.nanos
    }

    /// Total de horas inteiras (truncadas).
    pub fn as_hours(&self) -> u64 {
        self.nanos / Self::HOUR.nanos
    }

    /// Total de dias inteiros (truncados).
    pub fn as_days(&self) -> u64 {
        self.nanos / Self::DAY.nanos
    }

    /// True se a duração for zero.
    pub fn is_zero(&self) -> bool {
        self.nanos == 0
    }

    /// Representação canónica: `"3d2h30m15.500s"` ou `"0s"`.
    pub fn to_string(&self) -> String {
        if self.nanos == 0 {
            return "0s".to_string();
        }

        let mut rem = self.nanos;
        let days = rem / Self::DAY.nanos;
        rem %= Self::DAY.nanos;
        let hours = rem / Self::HOUR.nanos;
        rem %= Self::HOUR.nanos;
        let minutes = rem / Self::MINUTE.nanos;
        rem %= Self::MINUTE.nanos;
        let seconds = rem / NANOS_PER_SECOND;
        let nanos_frac = rem % NANOS_PER_SECOND;

        let mut parts = Vec::new();
        if days > 0 {
            parts.push(format!("{}d", days));
        }
        if hours > 0 {
            parts.push(format!("{}h", hours));
        }
        if minutes > 0 {
            parts.push(format!("{}m", minutes));
        }

        if seconds > 0 || nanos_frac > 0 || parts.is_empty() {
            if nanos_frac > 0 {
                let frac = format!("{:09}", nanos_frac)
                    .trim_end_matches('0')
                    .to_string();
                parts.push(format!("{}.{frac}s", seconds));
            } else {
                parts.push(format!("{}s", seconds));
            }
        }

        parts.join("")
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_zero() {
        assert_eq!(Duration::ZERO.to_string(), "0s");
        assert!(Duration::ZERO.is_zero());
    }

    #[test]
    fn duration_from_seconds() {
        let d = Duration::from_seconds(270180);
        assert_eq!(d.as_seconds(), 270180);
        assert_eq!(d.as_minutes(), 4503);
        assert_eq!(d.as_hours(), 75);
        assert_eq!(d.as_days(), 3);
    }

    #[test]
    fn duration_from_various_units() {
        let a = Duration::from_days(3)
            .nanos
            + Duration::from_hours(2).nanos
            + Duration::from_minutes(30).nanos;
        let d = Duration::from_nanos(a);
        assert_eq!(d.to_string(), "3d2h30m");
    }

    #[test]
    fn duration_to_string_canonical() {
        let d = Duration::from_days(3)
            .nanos
            + Duration::from_hours(2).nanos
            + Duration::from_minutes(30).nanos
            + Duration::from_seconds(15).nanos
            + 500_000_000;
        assert_eq!(Duration::from_nanos(d).to_string(), "3d2h30m15.5s");
    }

    #[test]
    fn duration_to_string_subsecond_only() {
        let d = Duration::from_nanos(500_000); // 0.5 ms
        assert_eq!(d.to_string(), "0.0005s");
    }

    #[test]
    fn duration_to_string_zero() {
        assert_eq!(Duration::default().to_string(), "0s");
    }

    #[test]
    fn duration_equality() {
        assert_eq!(Duration::from_seconds(60), Duration::from_minutes(1));
        assert_ne!(Duration::from_seconds(1), Duration::from_minutes(1));
    }

    #[test]
    fn duration_ordering() {
        assert!(Duration::from_seconds(1) < Duration::from_seconds(2));
        assert!(Duration::from_minutes(1) > Duration::from_seconds(1));
    }

    #[test]
    fn duration_copy() {
        let a = Duration::from_seconds(7);
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn duration_constants() {
        assert_eq!(Duration::SECOND.nanos, 1_000_000_000);
        assert_eq!(Duration::MINUTE.nanos, 60_000_000_000);
        assert_eq!(Duration::HOUR.nanos, 3_600_000_000_000);
        assert_eq!(Duration::DAY.nanos, 86_400_000_000_000);
    }
}
