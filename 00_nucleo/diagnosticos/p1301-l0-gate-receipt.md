# P1301 — recibo do gate L0

- **Estado:** `CONFIRMADO_PELO_DONO`
- **Confirmação textual:** `Autorizado`
- **Objeto confirmado:** abertura de passo dedicado de paridade de
  `compiler/eval/bindings/field_access`, atualização L0-first do owner 1:1 e
  nova cadeia de contrato, testes, implementação, ataques, selo e verificação.
- **Ordem causal:** a confirmação humana ocorreu após o relatório terminal
  P1300 e antes da medição/redação P1301.
- **Registro local materializado em:**
  `2026-09-03T20:59:16.447235803-03:00`.
- **Limitação temporal:** a interface conversacional não fornece a este
  executor o timestamp exato da mensagem humana; o horário acima é o instante
  de materialização do recibo, não um timestamp inventado da confirmação.

## L0 confirmado e redigido antes do código

| Prompt proprietário | SHA-256 após redação P1301 | Consumer |
|---|---|---|
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16` | `01_core/src/compiler/eval/bindings/field_access.rs` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a` | `01_core/src/compiler/eval/tests.rs` |

O gate autoriza a correção categórica de diagnóstico de field ausente em
`Module`, inclusive a projeção pública `std` → `global`; não autoriza mudar
`repr(std)`, entidade `Module`, API pública, fase, default ou criar blacklist
por nome de field.

Regime: protocolo completo Tekt, executado sem atestação de isolamento
técnico porque o filesystem dos executores é compartilhado.
