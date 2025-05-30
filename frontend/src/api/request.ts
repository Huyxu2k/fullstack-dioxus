import type { AxiosRequestConfig, Method } from 'axios';

import { message as $message } from 'antd';
import axios from 'axios';

import store from '@/stores';
import { setGlobalState } from '@/stores/global.store';
// import { history } from '@/routes/history';

const axiosInstance = axios.create({
  timeout: 6000,
});

axiosInstance.interceptors.request.use(
  config => {
    store.dispatch(
      setGlobalState({
        loading: true,
      }),
    );

    return config;
  },
  error => {
    store.dispatch(
      setGlobalState({
        loading: false,
      }),
    );
    Promise.reject(error);
  },
);

axiosInstance.interceptors.response.use(
  config => {
    store.dispatch(
      setGlobalState({
        loading: false,
      }),
    );

    if (config?.data?.message) {
      // $message.success(config.data.message)
    }

    return config?.data;
  },
  error => {
    store.dispatch(
      setGlobalState({
        loading: false,
      }),
    );
    // if needs to navigate to login page when request exception
    // history.replace('/login');
    let errorMessage = 'System exception';

    if (error?.message?.includes('Network Error')) {
      errorMessage = 'Network error, please check your network';
    } else {
      errorMessage = error?.message;
    }

    console.dir(error);
    error.message && $message.error(errorMessage);

    return {
      status: false,
      message: errorMessage,
      result: null,
    };
  },
);

export type Response<T = any> = {
  status: boolean;
  message: string;
  result: T;
};

export type MyResponse<T = any> = Promise<Response<T>>;

/**
 *
 * @param method - request methods
 * @param url - request url
 * @param data - request data or params
 */
export const request = <T = any>(
  method: Lowercase<Method>,
  url: string,
  data?: any,
  config?: AxiosRequestConfig,
): MyResponse<T> => {
  // const prefix = '/api'
  const prefix = '';
  //const prefix = 'http://127.0.0.1:8086/api/v1';

  url = prefix + url;

  switch (method) {
    case 'post':
      return axiosInstance.post(url, data, config);
    case 'get':
      return axiosInstance.get(url, { params: data, ...config });
    case 'delete':
      return axiosInstance.delete(url, { data, ...config });
    case 'put':
      return axiosInstance.put(url, data, config);
    case 'patch':
      return axiosInstance.patch(url, data, config);
    default:
      throw new Error(`Unsupported method: ${method}`);
  }
};
