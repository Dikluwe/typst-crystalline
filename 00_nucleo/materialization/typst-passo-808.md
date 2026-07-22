# Prompt — typst-passo-808: conferir citações remanescentes da regra revogada do Passo 48

**Origem**: recomendação #2 do handoff `handoff-novo-chat-p807.md`, nascida da medição de P800
**Estado**: aguardando execução

---

## Contexto

P800 revogou a regra do Passo 48 ("alinhar o eixo matemático à baseline do texto") por medição — o vanilla alinha a **baseline**, não o eixo; o eixo fica `axis_height` acima e só governa o centrado interno. A correcção foi feita em `equation.rs`, o L0 `equation.md` foi reescrito, e o teste que consagrava a regra antiga foi marcado `#[ignore]` (não apagado, com justificação).

O que P800 **não** cobriu: confirmar se a regra antiga está citada, assumida ou replicada em mais algum lugar do repositório — outro módulo de layout, outro L0, um ADR, um comentário de código, ou outro teste que dependa implicitamente do offset errado sem o testar directamente.

Este passo é uma varredura de confirmação, não uma correcção — só produz código se encontrar algo.

---

## Passo 1 — Varredura textual

1. Procurar no repositório (código, `00_nucleo/prompts/`, `00_nucleo/adr/`, `00_nucleo/DEBT.md`/`00_nucleo/diagnosticos/debt/DEBT.md`) por referências ao "Passo 48", a `axis_pt` usado como offset de baseline, e a qualquer descrição textual equivalente a "eixo matemático alinha com baseline do texto".
2. Para cada ocorrência: confirmar se é (a) já corrigida/anotada por P800, (b) uma citação histórica inofensiva (ex.: um relatório antigo descrevendo o que se fez na altura, que não precisa de mudar — factos históricos não se reescrevem), ou (c) uma citação activa que ainda assume a regra errada e precisa de correcção.

## Passo 2 — Varredura de comportamento

1. Procurar outros pontos do layout matemático que usem `axis_pt`/`axis_height` para posicionamento vertical fora do centrado interno já corrigido — especialmente equações em bloco (display) e não só inline, que P800 não teve como escopo testar directamente (o achado original era `$x^2$` inline).
2. Compilar um caso de equação em bloco (`$ x^2 $` sozinha numa linha, sintaxe de display) com os dois binários e comparar o alinhamento vertical por `mutool trace`, para confirmar que o bloco também está correcto ou, se não estiver, registar como achado novo (não corrigir aqui sem sonda própria — só registar).

## Passo 3 — Relatório

Produzir `00_nucleo/materialization/typst-passo-808-relatorio.md` com:
- Lista de todas as ocorrências encontradas no Passo 1, classificadas (a)/(b)/(c).
- Correcção de qualquer ocorrência (c), com comando + saída literal antes/depois.
- Resultado do teste de equação em bloco do Passo 2, com `mutool trace` ou equivalente.
- Se o Passo 2 revelar um achado novo (equação em bloco desalinhada), registar como achado separado, não corrigir dentro deste passo — este passo é de confirmação, um achado novo merece o seu próprio ciclo sonda→implementação→validação.
- Se nada for encontrado nos Passos 1 e 2, o relatório regista isso explicitamente (ausência de achado é também um resultado válido, desde que mostrado com a varredura feita).

## Regra da linha de trabalho (obrigatória, igual aos passos anteriores)

Toda afirmação deste relatório precisa de comando exacto + saída literal ou grep/busca reproduzível. "Não encontrei nada" só é aceitável se acompanhado do comando de busca usado.
