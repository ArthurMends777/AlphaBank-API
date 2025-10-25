-- Script de Correção - Alpha Bank Database
-- Execute este script no MySQL Workbench para corrigir os nomes das colunas

USE alpha_bank;

-- Opção 1: Adicionar colunas virtuais (GENERATED) que funcionam como aliases
-- Isso mantém a coluna 'type' original e cria aliases que o Rust espera

-- Corrigir tabela categories
ALTER TABLE categories 
ADD COLUMN category_type VARCHAR(20) 
GENERATED ALWAYS AS (type) VIRTUAL;

-- Corrigir tabela transactions
ALTER TABLE transactions 
ADD COLUMN transaction_type VARCHAR(20) 
GENERATED ALWAYS AS (type) VIRTUAL;

-- Verificar se funcionou
SELECT 'Categories corrigidas:' as status;
SELECT id, name, type, category_type FROM categories LIMIT 5;

SELECT 'Transactions corrigidas:' as status;
SELECT id, description, type, transaction_type FROM transactions LIMIT 5;

-- Pronto! Agora o código Rust vai funcionar sem modificações

