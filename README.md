Projet
---
WIP...

Routes 
---

### User

- [ ] **GET** `/api/v1/user` - Get all users
- [ ] **GET** `/api/v1/user/:id` - Get user by id
- [ ] **POST** `/api/v1/user` - Create a new user
- [ ] **PUT** `/api/v1/user/:id` - Update user by id
- [ ] **POST** `/api/v1/user/:id/upload-picture` - Update user profile picture by id
- [ ] **DELETE** `/api/v1/user/:id/upload-picture` - Delete user profile picture by id
- [ ] **DELETE** `/api/v1/user/:id` - Delete user by id
- [ ] **GET** `/api/v1/user/:id/house` - Get house by user id

### Authentification

- [ ] **POST** `/api/v1/auth/login` - Login
- [ ] **POST** `/api/v1/auth/register` - Register
- [ ] **POST** `/api/v1/auth/logout` - Logout
- [ ] **POST** `/api/v1/auth/refresh` - Refresh token
- [ ] **POST** `/api/v1/auth/forgot-password` - Forgot password
- [ ] **POST** `/api/v1/auth/reset-password` - Reset password
- [ ] **POST** `/api/v1/auth/verify-email` - Verify email
- [ ] **POST** `/api/v1/auth/change-password` - Change password

### House

- [ ] **GET** `/api/v1/house` - Get all houses
- [ ] **GET** `/api/v1/house/:id` - Get house by id
- [ ] **POST** `/api/v1/house` - Create a new house
- [ ] **PUT** `/api/v1/house/:id` - Update house by id
- [ ] **DELETE** `/api/v1/house/:id` - Delete house by id
- [ ] **GET** `/api/v1/house/:id/user` - Get user by house id
- **POST** `/api/v1/house/:id/user` - Add user to house by house id
- **DELETE** `/api/v1/house/:id/user/:userId` - Remove user from house by house id and user id
- **GET** `/api/v1/house/:id/expense` - Get expense by house id

### Expense

- **GET** `/api/v1/expense` - Get all expenses
- **GET** `/api/v1/expense/:id` - Get expense by id
- **POST** `/api/v1/expense` - Create a new expense
- **PUT** `/api/v1/expense/:id` - Update expense by id
- **DELETE** `/api/v1/expense/:id` - Delete expense by id

