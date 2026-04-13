//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math/symbols.md
//! @prompt-hash 9d0c2cee
//! @layer L1
//! @updated 2026-04-03

/// Converte um identificador matemático (ex: "alpha") para o carácter
/// Unicode correspondente, se existir na tabela.
/// Retorna `None` se o identificador não é um símbolo conhecido.
pub fn ident_to_unicode(name: &str) -> Option<&'static str> {
    match name {
        // Letras gregas minúsculas
        "alpha"    => Some("α"),
        "beta"     => Some("β"),
        "gamma"    => Some("γ"),
        "delta"    => Some("δ"),
        "epsilon"  => Some("ε"),
        "zeta"     => Some("ζ"),
        "eta"      => Some("η"),
        "theta"    => Some("θ"),
        "iota"     => Some("ι"),
        "kappa"    => Some("κ"),
        "lambda"   => Some("λ"),
        "mu"       => Some("μ"),
        "nu"       => Some("ν"),
        "xi"       => Some("ξ"),
        "pi"       => Some("π"),
        "rho"      => Some("ρ"),
        "sigma"    => Some("σ"),
        "tau"      => Some("τ"),
        "upsilon"  => Some("υ"),
        "phi"      => Some("φ"),
        "chi"      => Some("χ"),
        "psi"      => Some("ψ"),
        "omega"    => Some("ω"),
        // Letras gregas maiúsculas
        "Alpha"    => Some("Α"),
        "Beta"     => Some("Β"),
        "Gamma"    => Some("Γ"),
        "Delta"    => Some("Δ"),
        "Epsilon"  => Some("Ε"),
        "Theta"    => Some("Θ"),
        "Lambda"   => Some("Λ"),
        "Xi"       => Some("Ξ"),
        "Pi"       => Some("Π"),
        "Sigma"    => Some("Σ"),
        "Phi"      => Some("Φ"),
        "Psi"      => Some("Ψ"),
        "Omega"    => Some("Ω"),
        // Operadores e símbolos comuns
        "sum"      => Some("∑"),
        "prod"     => Some("∏"),
        "int"      => Some("∫"),
        "infty"    => Some("∞"),
        "partial"  => Some("∂"),
        "nabla"    => Some("∇"),
        "forall"   => Some("∀"),
        "exists"   => Some("∃"),
        "in"       => Some("∈"),
        "notin"    => Some("∉"),
        "subset"   => Some("⊂"),
        "supset"   => Some("⊃"),
        "union"    => Some("∪"),
        "inter"    => Some("∩"),
        "emptyset" => Some("∅"),
        "times"    => Some("×"),
        "div"      => Some("÷"),
        "pm"       => Some("±"),
        "mp"       => Some("∓"),
        "cdot"     => Some("·"),
        "dots"     => Some("…"),
        "ldots"    => Some("…"),
        "cdots"    => Some("⋯"),
        "vdots"    => Some("⋮"),
        "ddots"    => Some("⋱"),
        "approx"   => Some("≈"),
        "sim"      => Some("∼"),
        "cong"     => Some("≅"),
        "equiv"    => Some("≡"),
        "propto"   => Some("∝"),
        "perp"     => Some("⊥"),
        "parallel" => Some("∥"),
        "angle"    => Some("∠"),
        "circ"     => Some("∘"),
        "bullet"   => Some("•"),
        "star"     => Some("★"),
        "dagger"   => Some("†"),
        "hbar"     => Some("ℏ"),
        "ell"      => Some("ℓ"),
        "Re"       => Some("ℜ"),
        "Im"       => Some("ℑ"),
        "aleph"    => Some("ℵ"),
        _          => None,
    }
}

/// Converte um shorthand matemático (ex: "->") para o carácter Unicode.
///
/// Nota: o AST `MathShorthand::get()` já faz esta conversão — esta tabela
/// serve para testes directos e documentação da correspondência.
pub fn shorthand_to_unicode(text: &str) -> Option<&'static str> {
    match text {
        "=>"   => Some("⇒"),
        "==>"  => Some("⟹"),
        "<=>"  => Some("⇔"),
        "->>"  => Some("↠"),
        "->"   => Some("→"),
        "-->"  => Some("⟶"),
        "<-"   => Some("←"),
        "<--"  => Some("⟵"),
        "<->"  => Some("↔"),
        "<-->" => Some("⟷"),
        "|->|" => Some("↦"),
        "!="   => Some("≠"),
        "<="   => Some("≤"),
        ">="   => Some("≥"),
        "<<"   => Some("≪"),
        "<<<"  => Some("⋘"),
        ">>"   => Some("≫"),
        ">>>"  => Some("⋙"),
        ":="   => Some("≔"),
        "::="  => Some("⩴"),
        "=:"   => Some("≕"),
        "..."  => Some("…"),
        ".."   => Some("‥"),
        _      => None,
    }
}

/// Retorna `true` se o identificador é uma função matemática conhecida
/// (deve ser renderizado em texto normal, não itálico).
pub fn is_math_function(name: &str) -> bool {
    matches!(name,
        "sin" | "cos" | "tan" | "cot" | "sec" | "csc"      |
        "arcsin" | "arccos" | "arctan"                       |
        "sinh" | "cosh" | "tanh"                             |
        "log" | "ln" | "exp"                                 |
        "lim" | "limsup" | "liminf"                          |
        "max" | "min" | "sup" | "inf"                        |
        "det" | "tr" | "rank" | "dim" | "ker" | "im"        |
        "gcd" | "lcm" | "mod" | "div"                        |
        "Pr"  | "Var" | "Cov" | "E"  |
        "sqrt" | "root"
    )
}

/// Retorna `true` se o identificador é uma variável de uma letra
/// (deve ser renderizado em itálico matemático).
pub fn is_single_letter_var(name: &str) -> bool {
    name.len() == 1
        && name.chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false)
}

/// Retorna true se o caractere é um operador grande que deve receber
/// limites (sup/sub) empilhados verticalmente em vez de à direita.
pub fn is_large_operator(c: char) -> bool {
    matches!(c,
        // Somatório, produto, coproduto
        '∑' | '∏' | '∐' |
        // União, intersecção e variantes
        '⋃' | '⋂' | '⨄' | '⨅' | '⨆' |
        // Integrais
        '∫' | '∬' | '∭' | '∮' | '∯' | '∰' |
        // Outros operadores grandes comuns
        '⨁' | '⨂' | '⨀' | '⋀' | '⋁'
    )
}

/// Retorna true se o texto base de um MathIdent aceita limites verticais.
pub fn is_limit_function(s: &str) -> bool {
    matches!(s, "lim" | "max" | "min" | "sup" | "inf" | "limsup" | "liminf")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_converte_para_unicode() {
        assert_eq!(ident_to_unicode("alpha"), Some("α"));
    }

    #[test]
    fn sum_converte_para_unicode() {
        assert_eq!(ident_to_unicode("sum"), Some("∑"));
    }

    #[test]
    fn pi_converte_para_unicode() {
        assert_eq!(ident_to_unicode("pi"), Some("π"));
    }

    #[test]
    fn identificador_desconhecido_retorna_none() {
        assert_eq!(ident_to_unicode("foobar"), None);
    }

    #[test]
    fn shorthand_seta_direita() {
        assert_eq!(shorthand_to_unicode("->"), Some("→"));
    }

    #[test]
    fn shorthand_diferente() {
        assert_eq!(shorthand_to_unicode("!="), Some("≠"));
    }

    #[test]
    fn shorthand_implicacao() {
        assert_eq!(shorthand_to_unicode("=>"), Some("⇒"));
    }

    #[test]
    fn shorthand_desconhecido_retorna_none() {
        assert_eq!(shorthand_to_unicode("???"), None);
    }

    #[test]
    fn sin_e_funcao_nao_variavel() {
        assert!(is_math_function("sin"));
        assert!(!is_single_letter_var("sin"));
    }

    #[test]
    fn x_e_variavel_de_uma_letra() {
        assert!(is_single_letter_var("x"));
        assert!(!is_math_function("x"));
    }

    #[test]
    fn variavel_multi_letra_nao_e_single_letter() {
        assert!(!is_single_letter_var("xx"));
        assert!(!is_single_letter_var(""));
    }

    #[test]
    fn digit_nao_e_variavel() {
        assert!(!is_single_letter_var("1"));
    }

    // ── Passo 49 ─────────────────────────────────────────────────────────────

    #[test]
    fn is_large_operator_reconhece_sum() {
        assert!(is_large_operator('∑'));
    }

    #[test]
    fn is_large_operator_reconhece_prod() {
        assert!(is_large_operator('∏'));
    }

    #[test]
    fn is_large_operator_reconhece_integral() {
        assert!(is_large_operator('∫'));
    }

    #[test]
    fn is_large_operator_nao_reconhece_x() {
        assert!(!is_large_operator('x'));
    }

    #[test]
    fn is_large_operator_nao_reconhece_plus() {
        assert!(!is_large_operator('+'));
    }

    #[test]
    fn is_limit_function_reconhece_lim() {
        assert!(is_limit_function("lim"));
    }

    #[test]
    fn is_limit_function_reconhece_max_min() {
        assert!(is_limit_function("max"));
        assert!(is_limit_function("min"));
    }

    #[test]
    fn is_limit_function_reconhece_limsup_liminf() {
        assert!(is_limit_function("limsup"));
        assert!(is_limit_function("liminf"));
    }

    #[test]
    fn is_limit_function_nao_reconhece_sin() {
        assert!(!is_limit_function("sin"));
    }

    #[test]
    fn is_limit_function_nao_reconhece_x() {
        assert!(!is_limit_function("x"));
    }
}
