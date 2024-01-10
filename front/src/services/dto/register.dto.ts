export interface RegisterRequest {
    name: string;
    email: string;
    password: string;
}

export interface RegisterResponse {
    user_id: string | null;
    message: string;
    token: string | null;
}