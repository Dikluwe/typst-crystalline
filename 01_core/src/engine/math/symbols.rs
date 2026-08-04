//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/symbols.md
//! @prompt-hash e1e892f1
//! @layer L1
//! @updated 2026-04-03

/// Converte um identificador matemático (ex: "alpha") para o carácter
/// Unicode correspondente, se existir na tabela.
/// Retorna `None` se o identificador não é um símbolo conhecido.
pub fn ident_to_unicode(name: &str) -> Option<&'static str> {
    match name {
        // Letras gregas minúsculas
        "alpha" => Some("α"),
        "beta" => Some("β"),
        "gamma" => Some("γ"),
        "delta" => Some("δ"),
        "epsilon" => Some("ε"),
        "zeta" => Some("ζ"),
        "eta" => Some("η"),
        "theta" => Some("θ"),
        "iota" => Some("ι"),
        "kappa" => Some("κ"),
        "lambda" => Some("λ"),
        "mu" => Some("μ"),
        "nu" => Some("ν"),
        "xi" => Some("ξ"),
        "pi" => Some("π"),
        "rho" => Some("ρ"),
        "sigma" => Some("σ"),
        "tau" => Some("τ"),
        "upsilon" => Some("υ"),
        "phi" => Some("φ"),
        "chi" => Some("χ"),
        "psi" => Some("ψ"),
        "omega" => Some("ω"),
        // **P958** — minúsculos que faltavam (paridade codex `sym.txt`).
        "digamma" => Some("ϝ"),
        "omicron" => Some("ο"),
        // Letras gregas maiúsculas
        "Alpha" => Some("Α"),
        "Beta" => Some("Β"),
        "Gamma" => Some("Γ"),
        "Delta" => Some("Δ"),
        "Epsilon" => Some("Ε"),
        "Theta" => Some("Θ"),
        "Lambda" => Some("Λ"),
        "Xi" => Some("Ξ"),
        "Pi" => Some("Π"),
        "Sigma" => Some("Σ"),
        "Phi" => Some("Φ"),
        "Psi" => Some("Ψ"),
        "Omega" => Some("Ω"),
        // **P958** — maiúsculos que faltavam (paridade codex `sym.txt`).
        "Chi" => Some("Χ"),
        "Eta" => Some("Η"),
        "Iota" => Some("Ι"),
        "Kappa" => Some("Κ"),
        "Mu" => Some("Μ"),
        "Nu" => Some("Ν"),
        "Omicron" => Some("Ο"),
        "Rho" => Some("Ρ"),
        "Tau" => Some("Τ"),
        "Upsilon" => Some("Υ"),
        "Zeta" => Some("Ζ"),
        // Operadores e símbolos comuns
        "sum" => Some("∑"),
        // **P780** — `prod` não é nome de símbolo vanilla real (`codex`
        // `sym.txt` só tem `product ∏`; confirmado por compilação real:
        // `$product$` resolve a ∏ no vanilla, `$prod$` erra "unknown
        // variable: prod"). Mantido por compatibilidade retroativa
        // (nenhum teste depende dele activamente; risco de remoção não
        // avaliado — fora de âmbito deste passo). Adicionado o nome
        // correcto `product` a par, paridade `codex::sym.txt:525`.
        "prod" => Some("∏"),
        "product" => Some("∏"),
        // **P772w** — era `"int"` (errado: no vanilla, `int` é o construtor
        // do tipo inteiro — `scope.define("int", Value::Type(Type::Int))` em
        // `eval/mod.rs:1186` — e não está disponível directamente em modo
        // matemático; confirmado por compilação real do vanilla, que erra
        // "unknown variable: int" com hint "int is not available directly in
        // math ... use std.int"). O nome correcto do símbolo é `integral`
        // (paridade com `sym.rs::("integral", '∫', ...)`, a tabela completa
        // usada por `#sym.integral`). Bare `$integral$` produzia texto
        // literal "integral" em vez de ∫ antes desta correcção.
        "integral" => Some("∫"),
        "infty" => Some("∞"),
        // **P902** — U+1D715 (𝜕, itálico), não U+2202 (∂, upright).
        // Confirmado contra o vanilla real: `partial` produz sempre 𝜕,
        // não depende de contexto — ver teste
        // `p902_partial_converte_para_variante_italica` para justificação
        // completa (não é o mesmo mecanismo de itálico automático de
        // `alpha`/etc., é o codepoint directo na tabela).
        "partial" => Some("\u{1D715}"),
        "nabla" => Some("∇"),
        "forall" => Some("∀"),
        "exists" => Some("∃"),
        "in" => Some("∈"),
        "notin" => Some("∉"),
        "subset" => Some("⊂"),
        "supset" => Some("⊃"),
        "union" => Some("∪"),
        "inter" => Some("∩"),
        "emptyset" => Some("∅"),
        "times" => Some("×"),
        "div" => Some("÷"),
        "pm" => Some("±"),
        "mp" => Some("∓"),
        "cdot" => Some("·"),
        "dots" => Some("…"),
        "ldots" => Some("…"),
        "cdots" => Some("⋯"),
        "vdots" => Some("⋮"),
        "ddots" => Some("⋱"),
        "approx" => Some("≈"),
        "sim" => Some("∼"),
        "cong" => Some("≅"),
        "equiv" => Some("≡"),
        "propto" => Some("∝"),
        "perp" => Some("⊥"),
        "parallel" => Some("∥"),
        "angle" => Some("∠"),
        "circ" => Some("∘"),
        "bullet" => Some("•"),
        "star" => Some("★"),
        "dagger" => Some("†"),
        "hbar" => Some("ℏ"),
        "ell" => Some("ℓ"),
        "Re" => Some("ℜ"),
        "Im" => Some("ℑ"),
        "aleph" => Some("ℵ"),
        // **P812-D** — double-struck de conjuntos numéricos (paridade codex
        // `sym.txt`: NN ℕ, RR ℝ, ZZ ℤ, QQ ℚ, CC ℂ). Resto do alfabeto DS
        // (AA, BB, ...) — scope-out, adicionar on-demand.
        "NN" => Some("\u{2115}"),
        "RR" => Some("\u{211D}"),
        "ZZ" => Some("\u{2124}"),
        "QQ" => Some("\u{211A}"),
        "CC" => Some("\u{2102}"),
        _ => None,
    }
}

/// Converte um shorthand matemático (ex: "->") para o carácter Unicode.
///
/// Nota: o AST `MathShorthand::get()` já faz esta conversão — esta tabela
/// serve para testes directos e documentação da correspondência.
pub fn shorthand_to_unicode(text: &str) -> Option<&'static str> {
    match text {
        "=>" => Some("⇒"),
        "==>" => Some("⟹"),
        "<=>" => Some("⇔"),
        "->>" => Some("↠"),
        "->" => Some("→"),
        "-->" => Some("⟶"),
        "<-" => Some("←"),
        "<--" => Some("⟵"),
        "<->" => Some("↔"),
        "<-->" => Some("⟷"),
        "|->|" => Some("↦"),
        "!=" => Some("≠"),
        "<=" => Some("≤"),
        ">=" => Some("≥"),
        "<<" => Some("≪"),
        "<<<" => Some("⋘"),
        ">>" => Some("≫"),
        ">>>" => Some("⋙"),
        ":=" => Some("≔"),
        "::=" => Some("⩴"),
        "=:" => Some("≕"),
        "..." => Some("…"),
        ".." => Some("‥"),
        _ => None,
    }
}

/// Retorna `true` se o identificador é uma função matemática conhecida
/// (deve ser renderizado em texto normal, não itálico).
pub fn is_math_function(name: &str) -> bool {
    matches!(
        name,
        "sin"
            | "cos"
            | "tan"
            | "cot"
            | "sec"
            | "csc"
            | "arcsin"
            | "arccos"
            | "arctan"
            | "sinh"
            | "cosh"
            | "tanh"
            | "log"
            | "ln"
            | "exp"
            | "lim"
            | "limsup"
            | "liminf"
            | "max"
            | "min"
            | "sup"
            | "inf"
            | "det"
            | "tr"
            | "rank"
            | "dim"
            | "ker"
            | "im"
            | "gcd"
            | "lcm"
            | "mod"
            | "div"
            | "Pr"
            | "Var"
            | "Cov"
            | "E"
            | "sqrt"
            | "root"
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
///
/// **P772w** — inclui os caracteres de integral no conjunto (para spacing/
/// classe "Large"), mas quem decide limites empilhados (`attach.rs`) exclui
/// integrais via `is_integral_char` antes de usar este resultado — paridade
/// vanilla `Limits::for_char_with_class` (`MathClass::Large` só empilha se
/// `!is_integral_char(c)`; ver `math/attach.rs:166-174` no vanilla).
pub fn is_large_operator(c: char) -> bool {
    matches!(
        c,
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

/// Retorna `true` se o caractere é um sinal de integral — vanilla nunca
/// empilha limites (sup/sub) verticalmente para integrais, mesmo em modo
/// bloco/display (`Limits::Never` incondicional, `math/attach.rs:199-201` no
/// vanilla — `∫_0^1` mantém os scripts ao lado mesmo em display style,
/// diferente de `∑`/`∏`, que empilham em bloco). Faixas idênticas ao
/// vanilla: `'∫'..='∳'` (U+222B–U+2233) e `'⨋'..='⨜'` (U+2A0B–U+2A1C).
pub fn is_integral_char(c: char) -> bool {
    ('∫'..='∳').contains(&c) || ('⨋'..='⨜').contains(&c)
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

    /// **P958** — os 13 nomes gregos canónicos que faltavam face ao codex
    /// (`sym.txt`): minúsculos `digamma`/`omicron`; maiúsculos `Chi`, `Eta`,
    /// `Iota`, `Kappa`, `Mu`, `Nu`, `Omicron`, `Rho`, `Tau`, `Upsilon`,
    /// `Zeta`.
    #[test]
    fn p958_gregos_em_falta_convertem_para_unicode() {
        for (nome, glifo) in [
            ("digamma", "ϝ"),
            ("omicron", "ο"),
            ("Chi", "Χ"),
            ("Eta", "Η"),
            ("Iota", "Ι"),
            ("Kappa", "Κ"),
            ("Mu", "Μ"),
            ("Nu", "Ν"),
            ("Omicron", "Ο"),
            ("Rho", "Ρ"),
            ("Tau", "Τ"),
            ("Upsilon", "Υ"),
            ("Zeta", "Ζ"),
        ] {
            assert_eq!(
                ident_to_unicode(nome),
                Some(glifo),
                "{nome} deve resolver para {glifo} (paridade codex)"
            );
        }
    }

    /// **P902** — `partial` devia mapear para U+1D715 (𝜕, MATHEMATICAL
    /// ITALIC PARTIAL DIFFERENTIAL), não U+2202 (∂, PARTIAL DIFFERENTIAL
    /// upright). Confirmado por compilação directa contra o binário vanilla
    /// real (`lab/typst-original/target/release/typst`, ground truth):
    /// `$ partial $`/`$ partial x $`/`$ (partial f)/(partial x) $` produzem
    /// sempre `unicode="𝜕"` nos três casos, nunca `∂`. Não é
    /// dependente de contexto/itálico automático — confirmado comparando
    /// com `alpha` (que JÁ italiciza correctamente via
    /// `apply_math_default`/`is_math_italic_default`, aplicado a
    /// `MathText`/`MathIdent` de 1 carácter — mas exclui `∂`, que não é
    /// classificado como "letra" para esse efeito, ao contrário de `α`) e
    /// com `nabla`/`infty` (que ficam upright nos dois binários, confirmando
    /// que nem todo símbolo italiciza — `partial` é um caso à parte,
    /// resolvido directamente na tabela, mesmo padrão do `dot`/`⋅` corrigido
    /// em P894, não uma mudança na heurística de itálico automático).
    /// Cobertura de fonte confirmada nas 3 variantes de `NewCMMath` via
    /// `fontTools` antes da correcção (`typst-passo-902-relatorio.md`).
    #[test]
    fn p902_partial_converte_para_variante_italica() {
        assert_eq!(ident_to_unicode("partial"), Some("\u{1D715}")); // 𝜕
    }

    // ── P772w — símbolo `integral` (era `int`, nome errado) ────────────────

    #[test]
    fn integral_converte_para_unicode() {
        assert_eq!(ident_to_unicode("integral"), Some("∫"));
    }

    #[test]
    fn product_converte_para_unicode() {
        // P780 — paridade codex::sym.txt:525 (`product ∏`).
        assert_eq!(ident_to_unicode("product"), Some("∏"));
    }

    #[test]
    fn p812d_double_struck_conjuntos_numericos() {
        // P812-D — paridade vanilla (codex `sym.txt`): `$NN RR ZZ QQ CC$`
        // → `ℕℝℤℚℂ`. Antes: `unknown variable: NN`.
        assert_eq!(ident_to_unicode("NN"), Some("\u{2115}")); // ℕ
        assert_eq!(ident_to_unicode("RR"), Some("\u{211D}")); // ℝ
        assert_eq!(ident_to_unicode("ZZ"), Some("\u{2124}")); // ℤ
        assert_eq!(ident_to_unicode("QQ"), Some("\u{211A}")); // ℚ
        assert_eq!(ident_to_unicode("CC"), Some("\u{2102}")); // ℂ
        // Controlo: fora do conjunto continua desconhecido.
        assert_eq!(ident_to_unicode("AA"), None);
    }

    #[test]
    fn int_nao_e_simbolo_matematico() {
        // "int" é o construtor do tipo inteiro (`eval/mod.rs` scope global),
        // não um símbolo — vanilla erra "unknown variable: int" em modo
        // matemático (não está disponível directamente, só via `std.int`).
        // Antes de P772w, `ident_to_unicode("int")` devolvia `Some("∫")`
        // incorrectamente.
        assert_eq!(ident_to_unicode("int"), None);
    }

    #[test]
    fn is_integral_char_cobre_a_familia_de_integrais() {
        for c in ['∫', '∬', '∭', '∮', '∯', '∰', '∱', '∲', '∳'] {
            assert!(is_integral_char(c), "{c} deve ser reconhecido como integral");
        }
        assert!(!is_integral_char('∑'), "somatório não é integral");
        assert!(!is_integral_char('∏'), "produtório não é integral");
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
