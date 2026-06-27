# Relatório de refazimento — P447 DSM com `lente`

> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Motivo:** descoberta de que a ferramenta nativa do projeto para DSM é `lente`, não `cargo modules` nem `tekt dsm`.

---

## O que mudou

A primeira execução do P447 usou `cargo modules dependencies/export-json` como fallback, porque o binário `tekt` (referenciado no passo) não foi encontrado. Após verificação, descobriu-se que:

- o projecto **não tem** executável `tekt`;
- a ferramenta instalada é `~/.cargo/bin/lente`;
- `tekt-cargo-dsm` é apenas o nome da *lente* usada por `lente`, referenciada em `lab/parity/tools/decompor_so_vanilla.py`.

Como `lente` é a ferramenta própria do projeto e fornece:
- contagem exacta de módulos por crate,
- detecção de ciclos,
- dependências módulo → módulo,
- saída HTML interativa,

ela melhora a análise estrutural do audit. Por isso, o DSM foi regenerado.

---

## Artefactos novos

| Crate | Texto | HTML |
|-------|-------|------|
| `typst-core` | `00_nucleo/dsm/lente/typst-core.txt` | `00_nucleo/dsm/lente/typst-core.html` |
| `typst-shell` | `00_nucleo/dsm/lente/typst-shell.txt` | `00_nucleo/dsm/lente/typst-shell.html` |
| `typst-infra` | `00_nucleo/dsm/lente/typst-infra.txt` | `00_nucleo/dsm/lente/typst-infra.html` |
| `typst-wiring` | `00_nucleo/dsm/lente/typst-wiring.txt` | `00_nucleo/dsm/lente/typst-wiring.html` |

A análise consolidada foi actualizada em `00_nucleo/dsm/dsm-p447-analise.md`.

---

## Conclusão

A análise melhorou porque passou a usar a lente nativa do projeto. Os números de instabilidade são agora calculados ao nível de módulo (vista `lente`) e os ciclos foram explicitamente identificados. Nenhum ciclo novo foi introduzido por P445/P446. Os artefactos antigos (`dsm-p447-*.dot`/`dsm-p447-*.json`) ficam como proveniência do fallback inicial.
