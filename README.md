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
4. **Como um usuário**, quero configurar o bot para fazer compras automáticas (*sniping*) de tokens recém-lançados dos quais a carteria criadora tenha pelo menos 1 e não mais que x tokens lançados que superaram y de Market Cap ATH.
5. **Como um investidor**, quero que o agente de IA escaneie automaticamente o contrato inteligente da *memecoin* buscando funções maliciosas (como *mint infinito* ou taxas abusivas) para evitar a compra de *scams*.
6. **Como um trader**, quero receber alertas instantâneos no Telegram a cada operação de compra ou venda executada pelo bot para me manter informado sem precisar ficar olhando o painel.
7. **Como um usuário**, quero definir um valor fixo de Solana (SOL) a ser gasto por transação para manter um gerenciamento de risco rigoroso e consistente.
8. **Como um investidor**, quero poder exportar um relatório do meu histórico de Lucro e Perda (PNL) a qualquer momento para conferir meus rendimentos e facilitar minha declaração de impostos.
