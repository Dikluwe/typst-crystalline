# R1 — invalidação instrumental restrita

O reviewer apontou durante M1 que `shutil.copy2` preserva mtimes anteriores à
compilação. A observação posterior confirmou o risco: M1 compilou typst-core no
workspace exclusivo e falhou por `int` versus `integer`; M2 a M6 não recompilaram
e seus stdout são byte-idênticos ao de M1, apesar de fontes mutantes diferentes.

Assim, a classificação automática `valid_execution: true` / `failed_test` nos
recibos R1 de M2–M6 é INVALIDADA por este addendum: o veredito correto é `Unknown`,
causa `stale_compiled_mutant`. Nenhum desses cinco recebe crédito de mutação válida
ou eliminada. Seus recibos, logs e resultados agregados originais são preservados.

Controle C passou com fonte idêntica ao candidato já compilado pelo operador.
M1 possui compilação explícita de typst-core e testemunhas nominais Int em lookup
puro e AST; sua evidência permanece disponível ao reviewer.

Hipótese instrumental R2: atualizar apenas o mtime da fonte da cópia de execução
após cada copy2 força recompilação; exigir `Compiling typst-core`, registrar SHA
do executável e verificar a testemunha específica refuta reutilização silenciosa.
R2 executará somente M2–M6; plano, snapshots, testes e expectativas permanecem iguais.
Esta é a primeira revisão por causa de cache, sem revisão discriminatória do corpus.
