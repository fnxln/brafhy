export class LoginRequest {
    constructor(
        public username: string = '',
        public password: string = ''
    ) {}
    email: string;
    password: string;
}

export class LoginResponse {
    user_id: string;
    message: string;
    token: string;
}