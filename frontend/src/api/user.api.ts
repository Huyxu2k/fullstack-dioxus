import type { LoginParams, LoginResult, LogoutParams, LogoutResult } from '../interface/user/login';

import { request } from './request';


/** Login*/
export const apiLogin = (data: LoginParams) => request<LoginResult>('post', '/user/login', data);


/** Logout*/
export const apiLogout = (data: LogoutParams) => request<LogoutResult>('post', '/user/logout', data);
