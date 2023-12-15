package com.medwalletapp;

import android.os.Handler;
import android.util.Log;
import androidx.annotation.NonNull;
import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.util.RNLog;
import java.nio.charset.StandardCharsets;
import java.util.Base64;

public class RustBridgeModule extends ReactContextBaseJavaModule {
    private static ReactApplicationContext reactContext;
    // private static native int nativeAddNumbers(int a, int b);
    private static native long createAES();
    private static native byte[] encryptByteArray(long ptr, byte[] input);
    private static native byte[] decryptByteArray(long ptr, byte[] input);
    private static native void destroyAES(long aes_ptr);
    private static long aes_ptr;

    // Load the Rust library
    static {
        System.loadLibrary("fhe_enc");
        aes_ptr = createAES();
    }


    RustBridgeModule(ReactApplicationContext context) {
        super(context);
        reactContext = context;
    }



    @NonNull
    @Override
    public String getName() {
        return "RustModule";
    }

    // @ReactMethod
    // public void addNumbers(int a, int b, final Promise promise) {
    //     int result = nativeAddNumbers(a, b);
    //     promise.resolve(result);
    // }

    @ReactMethod
    public void encryptData(String input, final Promise promise) {
        // System.out.println("Plain text: " + input);
        //

        // base64 -> decode -> encrypt -> base64
        // String     byte[]    byte[]     String
        byte[] decoded = Base64.getDecoder().decode(input);
        byte[] cypher = encryptByteArray(aes_ptr, decoded);
        String encodedString = Base64.getEncoder().encodeToString(cypher);
        promise.resolve(encodedString);

        // promise.resolve(new String(cypher, StandardCharsets.UTF_8));

        // System.out.println("cypher: " + cypher);

        // String decrypted =
            // new String(decryptByteArray(aes_ptr, cypher), StandardCharsets.UTF_8);

        // System.out.println("Decrypted: " + decrypted);
        // destroyAES(aes_ptr);
    }


    @ReactMethod
    public void decryptData(String cypher, final Promise promise) {
        // base64 -> decode -> encrypt -> base64
        // String     byte[]    byte[]     String
        byte[] decoded = Base64.getDecoder().decode(cypher);
        byte[] decrypted = decryptByteArray(aes_ptr, decoded);
        String encodedString = Base64.getEncoder().encodeToString(decrypted);
        promise.resolve(encodedString);
        // promise.resolve(new String(decrypted, StandardCharsets.UTF_8));
        //
        // base64 -> decode -> decrypt -> base64

        // String encodedString = Base64.getDecoder().encodeToString(cypher);
        // System.out.println(encodedString);
        // promise.resolve(encodedString);
        //
        // String x = "Hello world";
        // promise.resolve(x);
        // Decoding
        // String decodedString = new String(decodedBytes);

        // System.out.println(decodedString); 
        // promise.resolve(decodedString);
    }
}
