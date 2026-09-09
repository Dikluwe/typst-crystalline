# P1336 — nota operacional do reviewer

Manifesto `54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`. No primeiro comando do reviewer, `git status --short` foi executado sem os pathspecs de exclusão e exibiu incidentalmente nomes de arquivos sob `00_nucleo/materialization/`. Isso não respeitou a restrição de não listar essa pasta. Nenhum conteúdo desses arquivos foi aberto ou usado para decidir a revisão. As leituras substantivas limitaram-se a L0, ADRs, skill, fonte e diagnósticos permitidos; os checkers posteriores excluem expressamente materialization/context.

Não se alega execução sem esse desvio operacional. A informação recebida foi de nomes/status, não conteúdo de instrução histórica nem oráculo/candidato privado entregue ao autor de testes. O regime permanece A/B sem atestação técnica de isolamento; esta nota não substitui os checks semânticos, de causalidade ou preservação e não confere autorização de acesso futuro às pastas restritas.
