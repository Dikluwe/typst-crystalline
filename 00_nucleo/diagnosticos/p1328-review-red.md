# P1328 — auditoria RED R1

Revisor `/root/p1328_review`; A/B sem atestação técnica de isolamento.
Recebido e auditado antes de C funcional.

Recibo `p1328-unit-red.json`, SHA-256 recalculado
`c58b10fb50c0ef95a658f0e520add9b18ec791660ecbd234fe98a5e53b89dfb3`,
manifesto `f7a0c4d6e7d6358fc5d34ff75465aca5da8d11188a2731e9275b9efbfe803098`.
Comando `cargo test --release --locked -p typst-core p1328 -- --nocapture`,
target `/tmp/p1328-target.T7Tg57`, executado de
`2026-09-09T11:44:40.991971+00:00` até
`2026-09-09T11:46:07.832324+00:00`, exit 101.

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093` mais working tree não
commitado, diff/stat e inventários completos estão no recibo. Os inventários
antes/depois registram a mesma fonte calc
`d7c753a83243caa6e5eed929698b9cda4c54277c05c20a8e82cb2924ce22d1e7`;
esse é o exato owner integrado auditado em `p1328-review-pre-c.md`.

O build terminou (`Finished release`) e executou o binário de unit tests.
Foram 7 testes: 5 falhas e 2 passes. Todas as 5 falhas atingiram igualdade
de mensagem, com recebido `calc.abs() requer Int ou Float, recebeu content`
contra o texto esperado congelado
`expected integer, float, length, angle, ratio, fraction, or decimal, found content`.
Os grupos de guards/números e números/rejeições intocadas passaram.
Não foi erro de compilação, fixture indisponível ou argumento de CLI inválido.

Veredito: RED semântico válido para R1. As falhas interrompem cada grupo
na primeira divergência; não se afirma que cada variante, rota ou perfil
interno já tenha sido atingido no RED. O GREEN deve executar esses mesmos
casos até o fim para verificar também os spans, wrappers e traces.

O erro de fmt informado separadamente refere-se à ordem de imports do
snippet no estilo do workspace. Um sucessor estritamente de formatação
deve ter delta mecânico demonstrado pelo autor A/B, novo hash e registro
de integração antes de C. Este RED permanece evidência histórica R1;
qualquer diferença semântica no sucessor exigiria reabrir o gate.
