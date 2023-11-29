package com.medwalletapp;

import android.os.Handler;
import android.util.Log;
import androidx.annotation.NonNull;
import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.util.RNLog;

public class RustBridgeModule extends ReactContextBaseJavaModule {
    // Load the Rust library
    static {
        System.loadLibrary("fhe_enc");
    }

    private static ReactApplicationContext reactContext;

    private static native int nativeAddNumbers(int a, int b);

    RustBridgeModule(ReactApplicationContext context) {
        super(context);
        reactContext = context;
    }

    @NonNull
    @Override
    public String getName() {
        return "RustModule";
    }

    @ReactMethod
    public void addNumbers(int a, int b, final Promise promise) {
        int result = nativeAddNumbers(a, b);
        promise.resolve(result);
    }
}
