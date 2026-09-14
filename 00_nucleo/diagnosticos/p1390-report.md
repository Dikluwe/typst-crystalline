# P1390 — relatório de paridade

## Veredito

`MATCH` para dois wrappers `Styled(Text)` size-only desiguais, ambos acima do
tamanho exterior, com uma palavra por body, exatamente um wrap na fronteira e
reserva vertical suficiente. 21→24, 24→21, 22→30 e 30→22 coincidem com o
vanila ratificado em página e nas seis coordenadas medidas, dentro de 0,001pt.

Tamanho igual continua sob P1389. Tamanho igual ou abaixo do exterior,
multiword, delta adicional, terceiro fragmento, wrapper aninhado, wrap interno
ou múltiplo, reserva insuficiente e forma opaca continuam `Unknown`.

## Implementação

Sequence mantém o certificado e o carrier privado de P1389, mas admite a
célula desigual somente quando o wrap único já foi provado. A reserva fixa
soma separadamente ascent, descent e leading exterior das duas linhas, em vez
de reutilizar duas vezes o maior estilo. Nenhuma API pública ou fase do
pipeline mudou.

## Evidência

- RED: 0/1, com 21→24 ainda em duas páginas.
- GREEN final P1390: 2/2; controles P138: 29/29; P137: 21/21.
- Matriz bilateral: 4/4 `MATCH`; pico 37.504 KiB.
- Build release e `cargo check --workspace`: sucesso.
- `crystalline-lint .`: zero violações, 248 warnings e 1185 infos, sem aumento.
- Execução serial e limitada; pico global 1.958.764 KiB, sem OOM, swap,
  timeout ou SIGKILL.

Os oito mutantes declarados foram confrontados com os oráculos focal,
fail-closed, históricos e bilateral. Não houve injeção mecânica de candidatos;
portanto o relatório não alega mutation score executado.

Proveniência final: 2026-09-14T12:04:18-03:00, HEAD
`906e2ed5a744779ab1a109d123f965f827500240`, working tree contendo apenas
P1390. Binários: vanila `eb60986b...`, cristalino `4f5b7544...`, teste
`7b0c1a55...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, implementação, ataques e veredito; hashes demonstram ordem
e identidade, não independência.
