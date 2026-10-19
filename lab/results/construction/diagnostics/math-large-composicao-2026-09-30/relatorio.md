# Operadores Large: escolha de variante e montagem

Estado medido: commit `17f13b1df79b87ea065ee62e78164b18e477da3d`, árvore de trabalho não commitada. Os arquivos alterados no início e no fim estão em `working-tree-start-status.txt`, `working-tree-final-status.txt` e respectivos `*-stat.txt`; horários UTC em `start-time.txt` e `end-time.txt`. Os SHA-256 finais estão em `final-sha256.txt`. O checkout é compartilhado com três outras frentes; não se atribuem a esta análise as alterações externas ao escopo matemático.

## Medição antes da decisão

No vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, `ir/resolve.rs:350-352` pede stretch vertical para Large em Display com alvo de `DisplayOperatorMinHeight` e short fall zero. `fragment/glyph.rs:265-300` conserva a base suficiente; depois seleciona variante suficiente; caso falhe, escolhe assembly disponível ou a maior variante. A implementação cristalina inicial em `math/layout/mod.rs:1712-1742` só selecionava a variante suficiente e voltava à base quando ela faltava. Isso não era apenas diferença mecânica: mudava a forma visual do operador.

Sonda controlada: `probe.typ` com NewCMMath-Book.otf do repositório. Para atingir os dois ramos sem alterar outras constantes, uma cópia da fonte foi criada em `/tmp` com somente `MATH.MathConstants.DisplayOperatorMinHeight=3000` via fontTools; SHA-256 da cópia `7309e4593ac90f72cb7bf0e3648fafd17dd107167f68876fbe013d3f9c1feb48`. O original tem upem=1000 e mínimo 1300du; `∑` tem variantes 1001/1401du sem assembly, `∫` tem variantes 1112/2223du com assembly. Vanilla compilado com `--ignore-system-fonts --ignore-embedded-fonts --font-path /tmp/tekt-math-large-2026-09-30`; cristalino usa `FONTCONFIG_FILE` restrito à mesma fonte e `--font-path`. Binários de entrada congelados e hashes em `oracle-sha256.txt`.

Antes: vanilla emitiu `∑` de avanço 1.444em e `∫` de três peças; cristalino emitiu `∑` 1.444em e `∫` variante 0.999em quando usava a fonte do sistema; isolada a fonte modificada, o cristalino antigo caiu para `∑` base 1.056em e `∫` base 0.665em. Depois da correção e do release build SHA-256 `7e3da2816c66f536f02ff6f196fa8d527171aa2d8685dd32e8cc96fe4f2a4f20`, `∑` voltou a 1.444em e `∫` usou as três peças. `vanilla-trace.txt` e `crystalline-after-trace.txt` registram os glifos e posições extraídos por `mutool draw -F trace`. As posições PDF das três peças são iguais até a precisão mostrada no trace (y 132.46613, 120.80063, 114.25012).

## Mudança e teste

L0 de `_comum.md` delega o ramo Large ao novo owner `large_operator.md`. O método saiu de `math/layout/mod.rs` para `math/layout/large_operator.rs`, sem despacho dinâmico ou reversão de imports. A política usa as variantes e a assembly fornecidas pela fonte; a geometria de montagem continua no owner `assembly.rs`. Foi adicionada a verificação da base suficiente antes da construção, também exigida pelo vanilla. Teste específico começou RED (`base-red.log`), ficou GREEN (`base-green.log`); 366 testes de `compiler::math::layout` passaram. `cargo build` e `cargo build --release` passaram. Linhagem foi ressellada só para `_comum.md`, `large_operator.md` e `tests.md`/seus três consumers; recibo em `scoped-seals.json` e receita em `seal_scope.py`.

`crystalline-lint . --checks v2,v5,v15,v26` não acusou V2/V5/V15/V26 em nenhum dos três consumers do escopo. O comando global saiu 1 por 12 V26 da pinagem de `paragraph-selection.toml` em outra frente do checkout compartilhado (`lint-errors.txt`). O lint global zero violações continua pendente dessa frente; nenhum reparo global de hashes foi executado aqui.

## Pendência observada no exportador

O PDF visual do integral montado coincide geometricamente, mas o vanilla insere `ActualText=∫` no conjunto de peças, enquanto a extração do cristalino devolve `⌡⎮⌠`. Os `FrameItem::Glyph` matemáticos carregam `base_char=∫`; a perda ocorre depois, no exportador PDF. Essa identidade textual é uma lacuna real de morfologia/acessibilidade, encaminhada à frente PDF. Não foi corrigida no owner de layout matemático para manter a separação de camadas e evitar sobreposição de trabalho no checkout.
