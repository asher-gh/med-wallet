package com.medwalletapp;

import android.os.Handler;
import androidx.annotation.NonNull;
import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;

public class RustBridgeModule extends ReactContextBaseJavaModule {

  private static ReactApplicationContext reactContext;

  private static native int nativeAddNumbers(int a, int b);

  // Load the Rust library
  static { System.loadLibrary("libfhe_enc"); }

  RustBridgeModule(ReactApplicationContext context) {
    super(context);
    reactContext = context;
  }

  @NonNull
  @Override
  public String getName() {
    return "LibFheEnc";
  }

  @ReactMethod
  public void addNumbers(int a, int b, final Promise promise) {
    int result = RustBridgeModule.nativeAddNumbers(a, b);
    promise.resolve(result);
  }
}
