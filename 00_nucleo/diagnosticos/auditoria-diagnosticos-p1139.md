# P1139 — Auditoria do formato público de diagnósticos

**Data:** 2026-08-23  
**HEAD:** `fcbc9763f8925d5c27b3670e35597b9adc0412a0`  
**Estado:** working tree não commitada; antes desta auditoria,
`typst-passo-1139.md` era o único ficheiro não rastreado.  
**Vanilla ratificado:** `upstream/main a51e02804`

## 1. Memória da decisão original

A ADR-0045 foi escrita em 2026-04-23, no Passo 111, para substituir o output
opaco `warning: Span(N) ...`. Naquele momento:

- o utilizador não recebia ficheiro, linha, coluna nem hints;
- a CLI real ainda não existia;
- o modelo era single-source;
- trace e resolução cross-file estavam fora do escopo;
- a ADR-0033 exigia paridade funcional, não visual;
- migrar o formatter vanilla completo foi considerado desproporcional.

O formato gcc/clang foi, portanto, uma solução incremental deliberada,
legível por humanos e parseável por editores. Não foi erro nem divergência
acidental.

## 2. Medição dos consumidores atuais

Comando:

```text
rg -n "format_diagnostic\s*\(" --glob '*.rs' \
  --glob '!lab/typst-original/**' --glob '!target/**' .
```

Resultado classificado:

| Classe | Quantidade | Local |
|---|---:|---|
| definição pública | 1 | `02_shell/src/diagnostic.rs` |
| consumidor de produção | 1 | `04_wiring/src/main.rs` |
| chamadas em testes unitários | 12 | `02_shell/src/diagnostic.rs` |

Não foi encontrado parser interno do formato gcc/clang. Os testes CLI
consultados verificam fragmentos semânticos (`error:`, `warning:`, mensagem,
path e ordem), não consomem a linha como protocolo estruturado. Como a função
é pública, consumidores externos desconhecidos continuam possíveis; o
repositório não fornece evidência para os enumerar.

## 3. Fonte vanilla medida

`lab/typst-original/crates/typst-kit/src/diagnostics.rs` define:

- `DiagnosticFormat::Human` como default;
- `DiagnosticFormat::Short` como formato curto alternativo;
- `codespan_reporting::term::emit` para o bloco humano;
- `tab_width = 2`;
- hints detached como notas `hint: ...`;
- hints com span como labels secundários;
- tracepoints emitidos depois do diagnóstico, resolvendo cada `FileId`.

O workspace vanilla pinado declara `codespan-reporting = "0.11"`; o lock
ratificado resolve `0.11.1` e `termcolor 1.4.1`.

## 4. Classificação

Segundo ADR-0107, a apresentação pública de um erro é o próprio observável:
mensagem, localização, snippet, marcador, hints e trace. Nos casos
`P1138-S-001..003`, a diferença não é estrutura interna de Rust nem bytes
privados; é feedback recebido pelo utilizador. A antiga classificação da
ADR-0033 era válida no seu contexto, mas foi superada para esta superfície.

## 5. Decisão proposta para o gate

1. O default passa a `Human`, espelhando o vanilla ratificado.
2. A implementação usa `codespan-reporting 0.11.1`, a mesma primitiva do
   baseline, em vez de reimplementar artesanalmente alinhamento e multiline.
3. L2 recebe fontes já carregadas, com `FileId`, `Source` e nome de exibição;
   não recebe `World` nem callback de I/O.
4. L4 resolve todas as fontes referidas pelo diagnóstico e pelos tracepoints.
5. O formato gcc/clang não permanece como modo público neste passo: não há
   flag `--diagnostic-format`, e criar uma mudaria novamente o produto.
6. A razão original da ADR-0045 permanece registrada; a revisão muda o default
   por evolução do critério de paridade, não por erro retroativo.

Esta decisão muda comportamento por defeito e API pública. Aplica-se a paragem
obrigatória da ADR-0127 antes dos testes RED e do código.
