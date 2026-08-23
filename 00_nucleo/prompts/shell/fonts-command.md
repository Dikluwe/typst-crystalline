# Prompt L0 — comando `typst fonts`
Hash do Código: b4ea5f27

**Camadas:** L2/L3/L4  
**Ficheiros alvo:** `02_shell/src/cli.rs`, `02_shell/src/fonts.rs`,
`03_infra/src/fonts.rs`, `04_wiring/src/main.rs`  
**Estado:** especificado; aguarda confirmação ADR-0127

## Medição anterior à decisão

O vanilla lista famílias descobertas e `--variants` detalha cada face
(`lab/.../fonts.rs:12-33`); combina fontes de sistema e `--font-path`
(`fonts.rs:36-54`). O cristalino já possui descoberta recursiva de TTF/OTF/
TTC/OTC em `03_infra/src/fonts.rs:202-217`, mas não expõe inventário formatado
nem comando.

**Classificação:** superfície CLI; a descoberta é mecânica L3, enquanto nomes,
ordem e variantes impressos são observáveis públicos.

## Contrato

```text
typst fonts [--font-path DIR ...] [--ignore-system-fonts] [--variants]
```

1. Sem `--variants`, uma família única por linha, em ordem determinística.
2. `--variants` lista path/origem, style, weight e stretch de cada face.
3. `--font-path` reutiliza descoberta L3 existente.
4. Por default inclui fontes do sistema; `--ignore-system-fonts` limita aos
   paths explícitos e fontes embutidas realmente disponíveis.
5. Fonte inválida é ignorada como na descoberta de compilação; path ilegível
   gera warning, não panic.

L3 devolve DTOs de inventário; L2 formata stdout; L4 somente compõe. Tipos de
`fontdb`/`ttf-parser` não vazam no contrato público de L2.

## Testes e aceitação

Fixture com duas famílias e coleção multiface; deduplicação; ordem estável;
`--variants`; ignore-system; path vazio/inválido. Comparar famílias e campos,
não paths absolutos específicos da máquina.

## Gate

Novo comando público: confirmação obrigatória ADR-0127.

