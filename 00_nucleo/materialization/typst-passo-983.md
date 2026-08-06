# Passo 983 — oráculo: split posicional (segundo emissor) para fechar o residual `TJ` de espaçamento de classe

**Precede este passo**: P982 confirmou, com números, que o residual `TJ` (79.7%/20.3% `Tj`/`TJ` no
oráculo, contra 92.5%/7.5% do vanilla) não é tolerância de arredondamento — é diferença genuína de
codificação. O cristalino escreve espaçamento de classe matemática (`THICK`=278‰, `MEDIUM`=222‰,
`THIN`=167‰ de em) como ajustes `TJ` dentro de um bloco de texto; o vanilla parte em blocos `Tm`
separados, com posição absoluta, exactamente nesses pontos. **Decisão do dono: implementar**, não
deixar como limitação aceite — a auditoria precisa deste nível de paridade para continuar a achar
detalhes reais sem ruído de formato.

**Continua restrito ao oráculo** (`03_infra/src/export/oracle.rs`) — não é mudança na saída
principal, mesma fronteira já estabelecida em P980/P982.

**Pré-condição de árvore**: `git status`. Confirmar P982 presente (sem código, só o relatório).

---

## Fase A — desenhar o split (gate obrigatório — o oráculo precisa de dados que hoje não tem)

1. Confirmar exactamente que dados faltam: P982 já apontou que o oráculo, operando só sobre a
   string do content stream já construído, não tem acesso aos avanços nominais da fonte
   (`glyph_to_nominal`) necessários para reposicionar cada pedaço com `Tm` absoluto depois de
   partir o run. Confirmar o caminho mais limpo para o oráculo receber essa informação — passá-la
   do builder no momento da chamada (`compile_to_pdf_bytes_oracle`), sem mudar o formato dos dados
   que o builder já produz para o caminho normal.
2. Confirmar a regra exacta de quando o vanilla parte um bloco: nos três valores de classe
   (THICK/MEDIUM/THIN) especificamente, ou em qualquer ajuste acima de um limiar? Ler
   `typst-pdf`/`krilla` de novo, desta vez focado em *quando* um novo `TextItem`/bloco é criado
   dentro de uma sequência matemática, não só na estrutura geral já lida em P979.
3. Desenhar o algoritmo: dado um run com ajustes `TJ` reais, identificar os pontos de corte
   (ajustes que correspondem a espaçamento de classe), calcular a posição absoluta de cada
   segmento resultante (via os avanços nominais), emitir cada segmento como o seu próprio
   `Tm`+`Tj`/`TJ` (recursivo — um segmento ainda pode ter ajustes internos que já são tratados por
   `collapse_trivial_tj`, P980).
4. Confirmar que isto não interage mal com o agrupamento de P979 (que fundiu os blocos na direcção
   oposta) — o oráculo está, neste ponto específico, a desfazer parcialmente esse agrupamento só
   nos pontos onde o vanilla também o desfaria. Não é regressão de P979 (que continua correcto na
   saída principal); é o oráculo a divergir de propósito da saída principal para se aproximar do
   vanilla.
5. Editar L0s, sincronizar hashes, **parar para confirmação do dono antes da Fase B**.

## Fase B — Implementação (protocolo de dois agentes de P898 — lógica nova de particionamento,
risco de introduzir posição errada se o split for feito incorrectamente)

1. Agente A escreve testes: um run com um único ajuste de classe THICK no meio — deve partir em
   dois blocos, com a posição do segundo calculada correctamente a partir do avanço nominal; um
   run sem ajuste de classe — não deve partir; um run com múltiplos pontos de corte — deve partir
   em N+1 segmentos, todos na posição certa.
2. Agente B implementa.
3. Revisão do orquestrador — confirmar, no documento de 30 secções, que a posição final de cada
   glifo depois do split é idêntica à da saída sem split (mesma prova de P979/P980 — zero glifos
   deslocados).
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Recontar `TJ`/`Tj` e `BT`/`ET` no documento de 30 secções — confirmar aproximação real dos
   1919 blocos e 92.5%/7.5% do vanilla, não só melhoria parcial.
2. `compare.py`/comparação por sequência de caractere — zero divergência de posição introduzida
   pelo split.
3. Confirmar que a saída principal (sem a flag do oráculo) continua bit-a-bit igual à de antes
   deste passo.
4. Benchmark do caminho normal — deve continuar em 1.00× exacto (nenhuma mudança de código no
   caminho sem a flag).

## Resultado esperado

- Oráculo particionando runs nos pontos de espaçamento de classe matemática, como o vanilla.
- Proporção `TJ`/`Tj` e contagem de `BT`/`ET` do oráculo muito mais próximas do vanilla.
- Posições de glifo inalteradas (prova, não suposição) — no oráculo e, sobretudo, confirmação de
  que a saída principal continua intocada.
- Ferramenta de auditoria mais confiável para os próximos passos de paridade.
