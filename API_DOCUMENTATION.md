# 📚 Alpha Bank API - Documentação Completa

## 🔐 Autenticação

Todas as rotas protegidas requerem o header:
```
Authorization: Bearer {seu_token_jwt}
```

---

## 📍 Endpoints

### 🔓 Públicos (Sem Autenticação)

#### 1. Registrar Usuário
**POST** `/api/auth/register`

**Body:**
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

**Resposta (201):**
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

**Body:**
```json
{
  "email": "joao@example.com",
  "password": "senha123"
}
```

**Resposta (200):**
```json
{
  "token": "eyJhbGc...",
  "user": { ... }
}
```

---

#### 3. Recuperar Senha
**POST** `/api/auth/forgot-password`

**Body:**
```json
{
  "email": "joao@example.com"
}
```

**Resposta (200):**
```json
{
  "message": "If the email exists, a recovery link will be sent"
}
```

---

### 🔒 Protegidos (Requerem Autenticação)

## 👤 Perfil do Usuário

#### 4. Obter Perfil
**GET** `/api/me`

**Resposta (200):**
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

**Body (todos os campos são opcionais):**
```json
{
  "full_name": "João Silva Santos",
  "email": "joao.novo@example.com",
  "phone": "(11) 99999-9999",
  "birth_date": "1990-01-15"
}
```

**Resposta (200):**
```json
{
  "message": "Profile updated successfully"
}
```

---

#### 6. Alterar Senha
**POST** `/api/auth/change-password`

**Body:**
```json
{
  "old_password": "senha123",
  "new_password": "novaSenha456"
}
```

**Resposta (200):**
```json
{
  "message": "Password changed successfully"
}
```

---

## 💰 Transações

#### 7. Listar Transações
**GET** `/api/transactions`

**Resposta (200):**
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

**Resposta (200):** Objeto de transação

---

#### 9. Criar Transação
**POST** `/api/transactions`

**Body:**
```json
{
  "description": "Compra no supermercado",
  "amount": 150.50,
  "transaction_type": "expense",
  "category_id": "uuid-da-categoria",
  "date": "2025-01-20"
}
```

**Resposta (201):** Objeto da transação criada

---

#### 10. Atualizar Transação
**PUT** `/api/transactions/{id}`

**Body (todos os campos opcionais):**
```json
{
  "description": "Compra no mercado (atualizado)",
  "amount": 175.00,
  "transaction_type": "expense",
  "category_id": "outro-uuid"
}
```

**Resposta (200):**
```json
{
  "message": "Transaction updated successfully"
}
```

---

#### 11. Deletar Transação
**DELETE** `/api/transactions/{id}`

**Resposta (200):**
```json
{
  "message": "Transaction deleted successfully"
}
```

---

## 🏷️ Categorias

#### 12. Listar Categorias
**GET** `/api/categories`

**Resposta (200):**
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

#### 13. Criar Categoria
**POST** `/api/categories`

**Body:**
```json
{
  "name": "Academia",
  "icon": "💪",
  "color": "#9b59b6",
  "category_type": "expense"
}
```

**Resposta (201):** Objeto da categoria criada

---

#### 14. Atualizar Categoria
**PUT** `/api/categories/{id}`

**Body (campos opcionais):**
```json
{
  "name": "Fitness",
  "icon": "🏋️",
  "color": "#8e44ad"
}
```

**Resposta (200):**
```json
{
  "message": "Category updated successfully"
}
```

---

#### 15. Deletar Categoria
**DELETE** `/api/categories/{id}`

**Resposta (200):**
```json
{
  "message": "Category deleted successfully"
}
```

---

## 🎯 Metas Financeiras

#### 16. Listar Metas
**GET** `/api/goals`

**Resposta (200):**
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

**Resposta (200):** Objeto da meta

---

#### 18. Criar Meta
**POST** `/api/goals`

**Body:**
```json
{
  "name": "Comprar carro",
  "target_amount": 50000.00,
  "deadline": "2026-06-30",
  "icon": "🚗"
}
```

**Resposta (201):** Objeto da meta criada

---

#### 19. Atualizar Meta
**PUT** `/api/goals/{id}`

**Body (campos opcionais):**
```json
{
  "name": "Comprar carro novo",
  "target_amount": 55000.00,
  "deadline": "2026-08-31"
}
```

**Resposta (200):**
```json
{
  "message": "Goal updated successfully"
}
```

---

#### 20. Adicionar Progresso à Meta
**POST** `/api/goals/{id}/progress`

**Body:**
```json
{
  "amount": 500.00
}
```

**Resposta (200):**
```json
{
  "message": "Progress added successfully"
}
```

---

#### 21. Deletar Meta
**DELETE** `/api/goals/{id}`

**Resposta (200):**
```json
{
  "message": "Goal deleted successfully"
}
```

---

## 🔄 Despesas Recorrentes

#### 22. Listar Recorrências
**GET** `/api/recurring`

**Resposta (200):**
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

#### 23. Criar Recorrência
**POST** `/api/recurring`

**Body:**
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

**Resposta (201):** Objeto da recorrência criada

---

#### 24. Atualizar Recorrência
**PUT** `/api/recurring/{id}`

**Body (campos opcionais):**
```json
{
  "description": "Spotify Family",
  "amount": 34.90,
  "active": false
}
```

**Resposta (200):**
```json
{
  "message": "Recurring transaction updated successfully"
}
```

---

#### 25. Deletar Recorrência
**DELETE** `/api/recurring/{id}`

**Resposta (200):**
```json
{
  "message": "Recurring transaction deleted successfully"
}
```

---

#### 26. Gerar Transações Pendentes
**POST** `/api/recurring/generate`

Gera automaticamente transações de todas as recorrências ativas que estão pendentes.

**Resposta (200):**
```json
{
  "message": "3 transactions generated",
  "count": 3
}
```

---

## 🔔 Notificações

#### 27. Listar Notificações
**GET** `/api/notifications`

**Resposta (200):**
```json
[
  {
    "id": "uuid",
    "user_id": "uuid",
    "title": "Meta atingida!",
    "message": "Você completou sua meta de viagem",
    "notification_type": "success",
    "read": false,
    "created_at": "2025-01-20T10:00:00Z"
  }
]
```

---

#### 28. Criar Notificação
**POST** `/api/notifications`

**Body:**
```json
{
  "title": "Lembrete",
  "message": "Pagar conta de luz",
  "notification_type": "warning"
}
```

**Tipos válidos:** `info`, `success`, `warning`, `error`

**Resposta (201):**
```json
{
  "id": "uuid",
  "message": "Notification created"
}
```

---

#### 29. Marcar como Lida
**PUT** `/api/notifications/{id}/read`

**Resposta (200):**
```json
{
  "message": "Notification marked as read"
}
```

---

#### 30. Deletar Notificação
**DELETE** `/api/notifications/{id}`

**Resposta (200):**
```json
{
  "message": "Notification deleted"
}
```

---

## 🏥 Health Check

#### 31. Verificar Status
**GET** `/health`

**Resposta (200):** `OK`

---

## 📊 Resumo de Endpoints

| Método | Endpoint | Descrição | Auth |
|--------|----------|-----------|------|
| POST | `/api/auth/register` | Registrar usuário | ❌ |
| POST | `/api/auth/login` | Login | ❌ |
| POST | `/api/auth/forgot-password` | Recuperar senha | ❌ |
| GET | `/api/me` | Obter perfil | ✅ |
| PUT | `/api/me` | Atualizar perfil | ✅ |
| POST | `/api/auth/change-password` | Alterar senha | ✅ |
| GET | `/api/transactions` | Listar transações | ✅ |
| POST | `/api/transactions` | Criar transação | ✅ |
| GET | `/api/transactions/{id}` | Buscar transação | ✅ |
| PUT | `/api/transactions/{id}` | Atualizar transação | ✅ |
| DELETE | `/api/transactions/{id}` | Deletar transação | ✅ |
| GET | `/api/categories` | Listar categorias | ✅ |
| POST | `/api/categories` | Criar categoria | ✅ |
| PUT | `/api/categories/{id}` | Atualizar categoria | ✅ |
| DELETE | `/api/categories/{id}` | Deletar categoria | ✅ |
| GET | `/api/goals` | Listar metas | ✅ |
| POST | `/api/goals` | Criar meta | ✅ |
| GET | `/api/goals/{id}` | Buscar meta | ✅ |
| PUT | `/api/goals/{id}` | Atualizar meta | ✅ |
| POST | `/api/goals/{id}/progress` | Adicionar progresso | ✅ |
| DELETE | `/api/goals/{id}` | Deletar meta | ✅ |
| GET | `/api/recurring` | Listar recorrências | ✅ |
| POST | `/api/recurring` | Criar recorrência | ✅ |
| PUT | `/api/recurring/{id}` | Atualizar recorrência | ✅ |
| DELETE | `/api/recurring/{id}` | Deletar recorrência | ✅ |
| POST | `/api/recurring/generate` | Gerar transações | ✅ |
| GET | `/api/notifications` | Listar notificações | ✅ |
| POST | `/api/notifications` | Criar notificação | ✅ |
| PUT | `/api/notifications/{id}/read` | Marcar como lida | ✅ |
| DELETE | `/api/notifications/{id}` | Deletar notificação | ✅ |
| GET | `/health` | Health check | ❌ |

**Total: 31 endpoints**

---

## 🚀 Como Testar

### PowerShell (Windows)

```powershell
# 1. Registrar
$register = @{
    full_name = "Teste"
    email = "teste@test.com"
    password = "123456"
    cpf = "123.456.789-00"
    birth_date = "1990-01-01"
    phone = "(11) 99999-9999"
} | ConvertTo-Json

$response = Invoke-WebRequest -Uri "http://localhost:8080/api/auth/register" `
    -Method POST -ContentType "application/json" -Body $register

$token = ($response.Content | ConvertFrom-Json).token

# 2. Usar o token
$headers = @{ "Authorization" = "Bearer $token" }

# 3. Listar transações
Invoke-WebRequest -Uri "http://localhost:8080/api/transactions" `
    -Headers $headers
```

---

## 📝 Códigos de Status HTTP

- **200 OK** - Sucesso
- **201 Created** - Recurso criado
- **400 Bad Request** - Dados inválidos
- **401 Unauthorized** - Não autenticado
- **403 Forbidden** - Sem permissão
- **404 Not Found** - Recurso não encontrado
- **409 Conflict** - Conflito (ex: email já existe)
- **500 Internal Server Error** - Erro no servidor

