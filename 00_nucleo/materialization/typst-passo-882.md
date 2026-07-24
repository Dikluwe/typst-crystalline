# Prompt — typst-passo-882: subsetting/embed das fontes latinas realmente usadas continua fraco (lorem e long)

**Origem**: P880 explicou por que `02-lorem` (4.22× o tamanho do vanilla) e `06-long` (6.64×) não encolheram como `04-math`/`05-tables` — o lazy coverage só evita carregar fontes **não usadas**; as fontes latinas que o documento de fato usa (Libertinus Serif) continuam sendo embutidas com nome genérico (`AAAAAA+CrystallineFont`) e stream maior que o vanilla, mesmo já passando pelo subsetting corrigido em P874.
**Estado**: aguardando execução

---

## Contexto: isto não é o mesmo problema do P797/P874

P874 já corrigiu o subsetting CFF para produzir CID-keyed válido (11× de redução em matemática). Este achado é diferente: mesmo com subsetting funcionando, o resultado para fontes latinas comuns ainda é maior que o vanilla — P880 mediu ~9.7KB contra ~8.2KB do vanilla para a mesma fonte em `02-lorem`, uma diferença pequena por ocorrência mas que se acumula (`06-long` tem muito mais texto, e a diferença vira 6.64×).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal. Contagem de testes discriminada por crate.

## Passo 1 — Sonda

1. Extrair a fonte embutida do PDF cristalino de `02-lorem` e do vanilla (mesmo método de P873/P874: `pdffonts`, `mutool extract`, `fontTools`) — comparar não só o tamanho total, mas a contagem de glifos embutidos nos dois. Confirmar se o cristalino já está de fato fazendo subset correto (só os glifos usados) ou se ainda sobra alguma coisa.
2. Se a contagem de glifos já bater entre os dois: a diferença de tamanho vem de outro lugar — tabelas extras na fonte subsetada (hinting, kerning, outras tabelas que o vanilla remove e o cristalino mantém), formato de compressão do stream PDF, ou overhead de metadados. Comparar a lista de tabelas presentes em cada fonte extraída.
3. Se a contagem de glifos não bater (cristalino embutindo mais glifos que o necessário mesmo com subsetting "funcionando"): confirmar por quê — pode ser um problema de over-inclusão no cálculo de quais glifos são "usados" (por exemplo, incluindo glifos de fallback que nunca aparecem no documento final).
4. Escalar o teste para `06-long` (mais texto, diferença mais visível) para confirmar que a causa é a mesma nos dois casos, não duas causas diferentes coincidindo.

## Passo 2 — Implementação

Depende do que o Passo 1 encontrar — pode ser remoção de tabelas desnecessárias do subset, correção do cálculo de glifos usados, ou ajuste de compressão. Não assumir a causa sem confirmar.

## Passo 3 — Validação

1. Repetir a extração de glifos/tabelas do Passo 1, confirmando redução de tamanho.
2. Repetir o benchmark de tamanho de PDF de P872/P880 para `02-lorem` e `06-long` — confirmar melhora na razão cristalino/vanilla.
3. Confirmar que o tempo de compilação (já bom nesses dois cenários, 0.35-0.44× e 1.20×) não regride.
4. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-882-relatorio.md` com: a comparação de glifos/tabelas entre cristalino e vanilla (Passo 1), a causa confirmada, o diff, e a medição de tamanho antes/depois para os dois cenários afetados.
