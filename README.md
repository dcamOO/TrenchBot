# Trenchbot 🚀

## 👥 Equipe

* **[Daniel Canton Alvim Moreira]** – Full Stack
* **[Thiago Felipe Alves do Carmo]** – Full Stack
* **[Gabriel Alves da Silva]** – Full Stack

## 🎯 Objetivo do Sistema

O **Trenchbot** é um agente autônomo de inteligência artificial projetado para operar na "trincheira" do mercado de criptomoedas, focando especificamente na negociação ágil de *memecoins* na blockchain da Solana. Seu objetivo é monitorar pools de liquidez em tempo real, identificar padrões em lançamentos de tokens e executar operações de compra e venda em frações de segundo. Utilizando a IA para análise de sentimento e métricas *on-chain*, o bot busca maximizar o lucro do usuário enquanto tenta identificar e mitigar os riscos de golpes (*rug pulls*).

## 🛠️ Tecnologias Utilizadas

* **Backend (Blockchain & Lógica Core):** Rust (para interação de altíssima performance com os *smart contracts* da Solana) e Python (para processamento de dados de mercado e lógica de IA).
* **Frontend:** React.js com Tailwind CSS (para um painel de controle simples, rápido e responsivo).
* **Banco de Dados:** PostgreSQL (para armazenamento estruturado do histórico de transações, logs e configurações do usuário).
* **Agente de IA:** Google Gemini (Agente Integrado) para análise de anomalias em contratos, tomada de decisão em cenários de alta volatilidade e otimização de estratégias de trade. 

## 📝 Histórias de Usuário

1. **Como um investidor**, quero conectar minha carteira da Solana (ex: Phantom) ao painel do bot para que ele possa executar as ordens de trade com meus fundos de forma segura.
2. **Como um usuário**, quero definir um limite de *stop-loss* e *take-profit* global para que o bot feche operações automaticamente e proteja meu capital.
3. **Como um trader**, quero visualizar um dashboard em tempo real com as moedas que o bot está monitorando no momento para acompanhar as oportunidades de mercado identificadas pela IA.
4. **Como um usuário**, quero configurar compras automáticas (*sniping*) de tokens recém-lançados de carteiras com pelo menos um lançamento anterior cujo Market Cap ATH superou Y, descartando imediatamente o token e a carteira se o total de tokens lançados pela carteira superar X.
5. **Como um investidor**, quero que o agente de IA escaneie automaticamente o contrato inteligente da *memecoin* buscando funções maliciosas (como *mint infinito* ou taxas abusivas) para evitar a compra de *scams*.
6. **Como um trader**, quero receber alertas instantâneos no Telegram a cada operação de compra ou venda executada pelo bot para me manter informado sem precisar ficar olhando o painel.
7. **Como um usuário**, quero definir um valor fixo de Solana (SOL) a ser gasto por transação para manter um gerenciamento de risco rigoroso e consistente.
8. **Como um investidor**, quero poder exportar um relatório do meu histórico de Lucro e Perda (PNL) a qualquer momento para conferir meus rendimentos e facilitar minha declaração de impostos.

## Motor de trading e simulação

As regras das histórias 2, 4 e 7 estão disponíveis em `src/trading`, com configuração JSON, replay local e descoberta ao vivo pelo WebSocket do PumpPortal. **As ordens são simuladas**: não conecta carteira nem envia swaps reais. Apenas lançamentos pump.fun são candidatos. O fluxo anterior de análise continua disponível ao executar sem argumentos.

```sh
cargo test
cargo run -- --paper examples/trading.json examples/events.jsonl
# Para consumir continuamente eventos JSONL pela entrada padrão:
cargo run -- --paper examples/trading.json -
# Descoberta ao vivo pump.fun e ordens simuladas (requer PUMPPORTAL_API_KEY):
cargo run -- --pump-paper examples/trading.json examples/histories.json
```

O exemplo compra dois tokens, fecha um por take-profit e outro por stop-loss. A saída contém um resultado JSON por evento. Erros de evento/ordem são reportados e o processamento continua, permitindo avaliar saídas posteriores. Configuração inválida interrompe a inicialização. O saldo inicial simulado é 10 SOL, alterável por `PAPER_BALANCE_LAMPORTS` (inteiro).

### Configuração

Edite `examples/trading.json` ou forneça outro arquivo:

| Campo | Regra |
| --- | --- |
| `stop_loss_percent` | Percentual de perda por posição, maior que 0 e até 100. |
| `take_profit_percent` | Percentual de ganho por posição, positivo. |
| `sol_per_trade` | Valor fixo por compra em SOL, como string decimal (`"0.1"`), com até 9 casas; convertido exatamente em lamports. |
| `sniping_enabled` | Habilita novas compras; desabilitá-lo mantém as saídas automáticas. |
| `max_creator_launches` | X: máximo de **todos** os tokens lançados pela carteira, incluindo o candidato, independentemente do ATH. |
| `min_ath_market_cap_usd` | Y em USD: pelo menos um token anterior deve ter ATH estritamente maior que Y. |
| `max_launch_age_seconds` | Idade máxima inclusiva do lançamento em relação ao instante de observação. |

Se o total de lançamentos ultrapassar X, o motor descarta imediatamente o candidato e bloqueia novos candidatos da carteira durante a sessão. Mesmo um histórico parcial pode provar esse excesso. Para aprovar uma compra, o histórico deve ser completo e pertencer ao criador informado. Registros duplicados são rejeitados; o candidato pode aparecer no histórico e é contado uma única vez. O candidato e tokens lançados depois dele não servem como sucesso anterior. ATH ausente/inválido impede a compra; market cap atual não substitui ATH.

Stop-loss e take-profit são configurações globais **aplicadas individualmente a todas as posições**, em relação ao custo de entrada em SOL. Ao atingir ou ultrapassar um limite, o motor solicita a venda de toda a quantidade. A função `Engine::configure` valida alterações e as aplica também às posições existentes. Carteiras bloqueadas continuam bloqueadas na sessão após reconfiguração; o bloqueio não impede saídas de posições abertas.

Cada compra usa o mesmo valor nominal em lamports. Saldo insuficiente impede a operação. A simulação não modela taxas, slippage, liquidez nem arredondamento por decimais do token. A detecção de saída depende da chegada de eventos de preço; não garante execução no preço do limite.

### Integração de dados e execução

`Event::Launch` recebe `platform` (apenas `pump_fun` é aceito), mint, carteira que realizou o lançamento, timestamps Unix em segundos, preço em SOL por token e histórico de deployments/ATH. `Event::Price` recebe mint, timestamp e preço em SOL; cotações fora de ordem são ignoradas. Veja o contrato JSON em `examples/events.jsonl`. Os identificadores `DEMO-*` são fictícios.

Um adaptador de dados real deverá verificar a carteira que efetivamente criou o token, obter todo o histórico (inclusive paginação), fornecer ATH em USD e produzir eventos recentes. Os campos atuais de metadata/Jupiter não fornecem esse histórico e não são usados para inferi-lo. Em replay histórico, o provedor deve fornecer somente dados conhecidos naquele momento.

`Broker` separa as regras da execução. Um adaptador real deverá assinar/enviar swaps, usar o valor exato solicitado e retornar apenas fills confirmados, com a quantidade efetivamente recebida. Erros devem significar que nenhuma operação ocorreu; resultados ambíguos precisam ser reconciliados pelo adaptador antes de retornar. Ordens com erro preservam o estado para nova tentativa. Tokens comprados não são recomprados na mesma sessão, mesmo após a venda.

Posições, tokens comprados e carteiras descartadas ficam em memória nesta versão. O modo de simulação começa uma sessão nova a cada execução. Persistência e reconciliação com a carteira, além dos adaptadores de dados e swaps, ainda são necessárias para operação real.

### WebSocket pump.fun

`--pump-paper` usa uma única conexão com `wss://pumpportal.fun/api/data`, assina `subscribeNewToken` e aceita somente eventos `pool: "pump"`. Ao abrir uma posição simulada, assina `subscribeTokenTrade` no mesmo socket; após a venda, cancela essa assinatura. A carteira criadora vem de `traderPublicKey` exclusivamente em eventos `txType: "create"`; compradores posteriores não são tratados como criadores. O preço estimado usa a razão entre reservas virtuais SOL/token.

A conexão é refeita com espera progressiva até 30 segundos, repondo as assinaturas das posições abertas. Qualquer reconexão invalida a completude dos históricos: novas compras ficam bloqueadas, mas o monitoramento das posições existentes continua. Para voltar a comprar, inicie outra sessão com histórico atualizado. Nenhuma chave aparece nos logs de conexão.

O arquivo de históricos é um objeto indexado pela carteira criadora:

```json
{
  "CARTEIRA_CRIADORA": {
    "creator": "CARTEIRA_CRIADORA",
    "complete": true,
    "tokens": [
      { "mint": "TOKEN_ANTERIOR", "launched_at": 1700000000, "ath_market_cap_usd": 2000000.0 }
    ]
  }
}
```

Esse snapshot deve ser completo e atualizado no início da sessão, fornecido por uma fonte confiável. Carteiras ausentes recebem histórico incompleto e não são aprovadas. Novos deployments observados se acumulam, inclusive os rejeitados, sem contar mensagens repetidas duas vezes. Seus ATHs em USD não são inferidos do preço atual; apenas o snapshot fornece sucessos comprovados. O exemplo `histories.json` está vazio propositalmente e não permite compras.

O WebSocket fornece dados ao vivo, não o histórico de deployments/ATH ([FAQ do provedor](https://pumpportal.fun/FAQ/)). Além disso, assinaturas de trades exigem chave e têm cobrança do provedor, mesmo com ordens simuladas ([preços](https://pumpportal.fun/fees/)). A descoberta de novos tokens é gratuita.

O adaptador cobre a curva pump.fun; preços após migração para PumpSwap ainda não são tratados. O instante de recebimento é usado como timestamp do evento ao vivo. O stream é `processed`, não uma confirmação final de blockchain. Esses limites, a ausência de persistência e os fills simulados impedem tratar esse modo como execução real de produção.
