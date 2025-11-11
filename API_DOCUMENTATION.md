# 📚 Documentação da API Alpha Bank

## 🔐 Autenticação

Todas as rotas protegidas exigem o envio do seguinte cabeçalho (Header):
```
Authorization: Bearer {seu_token_jwt}
```

---

## 📍 Endpoints da API

### 🔓 Rotas Públicas (Sem Autenticação)

#### 1. Registro de Usuário
**POST** `/api/auth/register`

**Corpo da Requisição (Body):**
```json
{
  "full_name": "João da Silva",
  "email": "joao@example.com",
  "password": "senha123",
  "cpf": "123.456.789-00",
  "birth_date": "1990-01-15",
  "phone": "(11) 98765-4321"
}
```

**Resposta (201 Created):**
```json
{
  "token": "eyJhbGc...",
  "user": {
    "id": "uuid",
    "full_name": "João da Silva",
    "email": "joao@example.com",
    ...
  }
}
```

---

#### 2. Login
**POST** `/api/auth/login`

**Corpo da Requisição (Body):**
```json
{
  "email": "joao@example.com",
  "password": "senha123"
}
```

**Resposta (200 OK):**
```json
{
  "token": "eyJhbGc...",
  "user": { ... }
}
```

---

#### 3. Recuperação de Senha
**POST** `/api/auth/forgot-password`

**Corpo da Requisição (Body):**
```json
{
  "email": "joao@example.com"
}
```

**Resposta (200 OK):**
```json
{
  "message": "Se o e-mail estiver cadastrado, um link de recuperação será enviado."
}
```

---

### 🔒 Rotas Protegidas (Requerem Autenticação)

## 👤 Gerenciamento de Perfil

#### 4. Obter Detalhes do Perfil
**GET** `/api/me`

**Resposta (200 OK):**
```json
{
  "id": "uuid",
  "full_name": "João da Silva",
  "email": "joao@example.com",
  "cpf": "123.456.789-00",
  "birth_date": "1990-01-15",
  "phone": "(11) 98765-4321",
  "created_at": "2025-01-01T00:00:00Z"
}
```

---

#### 5. Atualizar Perfil
**PUT** `/api/me`

**Corpo da Requisição (Body - todos os campos são opcionais):**
```json
{
  "full_name": "João Silva Santos",
  "email": "joao.novo@example.com",
  "phone": "(11) 99999-9999",
  "birth_date": "1990-01-15"
}
```

**Resposta (200 OK):**
```json
{
  "message": "Perfil atualizado com sucesso."
}
```

---

#### 6. Alterar Senha
**POST** `/api/auth/change-password`

**Corpo da Requisição (Body):**
```json
{
  "old_password": "senha123",
  "new_password": "novaSenha456"
}
```

**Resposta (200 OK):**
```json
{
  "message": "Senha alterada com sucesso."
}
```

---

## 💰 Transações Financeiras

#### 7. Listar Transações
**GET** `/api/transactions`

**Resposta (200 OK):**
```json
[
  {
    "id": "uuid",
    "user_id": "uuid",
    "description": "Salário",
    "amount": 5000.00,
    "transaction_type": "income",
    "category_id": "uuid",
    "date": "2025-01-15T00:00:00Z",
    "recurring": false,
    "recurring_id": null,
    "created_at": "2025-01-15T10:00:00Z"
  }
]
```

---

#### 8. Buscar Transação por ID
**GET** `/api/transactions/{id}`

**Resposta (200 OK):** Retorna o objeto da transação.

---

#### 9. Criar Nova Transação
**POST** `/api/transactions`

**Corpo da Requisição (Body):**
```json
{
  "description": "Compra no supermercado",
  "amount": 150.50,
  "transaction_type": "expense",
  "category_id": "uuid-da-categoria",
  "date": "2025-01-20"
}
```

**Resposta (201 Created):** Retorna o objeto da transação criada.

---

#### 10. Atualizar Transação
**PUT** `/api/transactions/{id}`

**Corpo da Requisição (Body - todos os campos opcionais):**
```json
{
  "description": "Compra no mercado (atualizado)",
  "amount": 175.00,
  "transaction_type": "expense",
  "category_id": "outro-uuid"
}
```

**Resposta (200 OK):**
```json
{
  "message": "Transação atualizada com sucesso."
}
```

---

#### 11. Excluir Transação
**DELETE** `/api/transactions/{id}`

**Resposta (200 OK):**
```json
{
  "message": "Transação excluída com sucesso."
}
```

---

## 🏷️ Categorias

#### 12. Listar Categorias
**GET** `/api/categories`

**Resposta (200 OK):**
```json
[
  {
    "id": "uuid",
    "user_id": null,
    "name": "Alimentação",
    "icon": "🍔",
    "color": "#e74c3c",
    "category_type": "expense",
    "is_default": true,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

---

#### 13. Criar Nova Categoria
**POST** `/api/categories`

**Corpo da Requisição (Body):**
```json
{
  "name": "Academia",
  "icon": "💪",
  "color": "#9b59b6",
  "category_type": "expense"
}
```

**Resposta (201 Created):** Retorna o objeto da categoria criada.

---

#### 14. Atualizar Categoria
**PUT** `/api/categories/{id}`

**Corpo da Requisição (Body - campos opcionais):**
```json
{
  "name": "Fitness",
  "icon": "🏋️",
  "color": "#8e44ad"
}
```

**Resposta (200 OK):**
```json
{
  "message": "Categoria atualizada com sucesso."
}
```

---

#### 15. Excluir Categoria
**DELETE** `/api/categories/{id}`

**Resposta (200 OK):**
```json
{
  "message": "Categoria excluída com sucesso."
}
```

---

## 🎯 Metas Financeiras

#### 16. Listar Metas
**GET** `/api/goals`

**Resposta (200 OK):**
```json
[
  {
    "id": "uuid",
    "user_id": "uuid",
    "name": "Viagem para Europa",
    "target_amount": 10000.00,
    "current_amount": 3500.00,
    "deadline": "2025-12-31",
    "icon": "✈️",
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-01-15T00:00:00Z"
  }
]
```

---

#### 17. Buscar Meta por ID
**GET** `/api/goals/{id}`

**Resposta (200 OK):** Retorna o objeto da meta.

---

#### 18. Criar Nova Meta
**POST** `/api/goals`

**Corpo da Requisição (Body):**
```json
{
  "name": "Comprar carro",
  "target_amount": 50000.00,
  "deadline": "2026-06-30",
  "icon": "🚗"
}
```

**Resposta (201 Created):** Retorna o objeto da meta criada.

---

#### 19. Atualizar Meta
**PUT** `/api/goals/{id}`

**Corpo da Requisição (Body - campos opcionais):**
```json
{
  "name": "Comprar carro novo",
  "target_amount": 55000.00,
  "deadline": "2026-08-31"
}
```

**Resposta (200 OK):**
```json
{
  "message": "Meta atualizada com sucesso."
}
```

---

#### 20. Adicionar Progresso à Meta
**POST** `/api/goals/{id}/progress`

**Corpo da Requisição (Body):**
```json
{
  "amount": 500.00
}
```

**Resposta (200 OK):**
```json
{
  "message": "Progresso adicionado com sucesso."
}
```

---

#### 21. Excluir Meta
**DELETE** `/api/goals/{id}`

**Resposta (200 OK):**
```json
{
  "message": "Meta excluída com sucesso."
}
```

---

## 🔄 Transações Recorrentes

#### 22. Listar Recorrências
**GET** `/api/recurring`

**Resposta (200 OK):**
```json
[
  {
    "id": "uuid",
    "user_id": "uuid",
    "description": "Netflix",
    "amount": 49.90,
    "transaction_type": "expense",
    "category_id": "uuid",
    "frequency": "monthly",
    "active": true,
    "last_generated": "2025-01-01T00:00:00Z",
    "created_at": "2024-12-01T00:00:00Z",
    "updated_at": "2025-01-01T00:00:00Z"
  }
]
```

---

#### 23. Criar Nova Recorrência
**POST** `/api/recurring`

**Corpo da Requisição (Body):**
```json
{
  "description": "Spotify Premium",
  "amount": 21.90,
  "transaction_type": "expense",
  "category_id": "uuid",
  "frequency": "monthly"
}
```

**Frequências válidas:** `daily`, `weekly`, `monthly`, `yearly`

**Resposta (201 Created):** Retorna o objeto da recorrência criada.

---

#### 24. Atualizar Recorrência
**PUT** `/api/recurring/{id}`

**Corpo da Requisição (Body - campos opcionais):**
```json
{
  "description": "Spotify Family",
  "amount": 34.90,
  "active": false
}
```

**Resposta (200 OK):**
```json
{
  "message": "Transação recorrente atualizada com sucesso."
}
```

---

#### 25. Excluir Recorrência
**DELETE** `/api/recurring/{id}`

**Resposta (200 OK):**
```json
{
  "message": "Transação recorrente excluída com sucesso."
}
```

---

#### 26. Gerar Transações Pendentes
**POST** `/api/recurring/generate`

Esta rota gera automaticamente transações para todas as recorrências ativas que estão pendentes.

**Resposta (200 OK):**
```json
{
  "message": "3 transações geradas",
  "count": 3
}
```

---

## 🏦 Conta Bancária

#### 27. Obter Saldo
**GET** `/api/account/balance`

**Resposta (200 OK):**
```json
{
  "balance": 15000.00
}
```

---

#### 28. Realizar Depósito
**POST** `/api/account/deposit`

**Corpo da Requisição (Body):**
```json
{
  "amount": 1000.00
}
```

**Resposta (200 OK):**
```json
{
  "message": "Depósito realizado com sucesso."
}
```

---

#### 29. Realizar Transferência
**POST** `/api/account/transfer`

**Corpo da Requisição (Body):**
```json
{
  "recipient_email": "destino@example.com",
  "amount": 500.00
}
```

**Resposta (200 OK):**
```json
{
  "message": "Transferência realizada com sucesso."
}
```

---

## 🔔 Notificações

#### 30. Listar Notificações
**GET** `/api/notifications`

**Resposta (200 OK):**
```json
[
  {
    "id": "uuid",
    "user_id": "uuid",
    "message": "Sua meta 'Viagem' atingiu 50% do progresso!",
    "read": false,
    "created_at": "2025-01-20T10:00:00Z"
  }
]
```

---

#### 31. Marcar Notificação como Lida
**PUT** `/api/notifications/{id}/read`

**Resposta (200 OK):**
```json
{
  "message": "Notificação marcada como lida."
}
```

---

#### 32. Obter Contagem de Não Lidas
**GET** `/api/notifications/unread_count`

**Resposta (200 OK):**
```json
{
  "count": 5
}
```

---

## 📊 Estatísticas

#### 33. Obter Estatísticas Mensais
**GET** `/api/stats/monthly`

**Resposta (200 OK):**
```json
{
  "current_month": {
    "income": 5500.00,
    "expense": 2100.00,
    "balance": 3400.00
  },
  "last_month": {
    "income": 5000.00,
    "expense": 2000.00,
    "balance": 3000.00
  }
}
```

---

#### 34. Obter Estatísticas por Categoria
**GET** `/api/stats/categories`

**Resposta (200 OK):**
```json
[
  {
    "category_name": "Alimentação",
    "total_expense": 800.00
  },
  {
    "category_name": "Transporte",
    "total_expense": 400.00
  }
]
```

---

#### 35. Obter Histórico de Fluxo de Caixa
**GET** `/api/stats/flow`

**Resposta (200 OK):**
```json
[
  {
    "date": "2025-01-01",
    "income": 100.00,
    "expense": 50.00
  },
  {
    "date": "2025-01-02",
    "income": 0.00,
    "expense": 75.00
  }
]
```
