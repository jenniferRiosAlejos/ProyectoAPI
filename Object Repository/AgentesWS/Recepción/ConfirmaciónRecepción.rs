<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ConfirmaciónRecepción</name>
   <tag></tag>
   <elementGuidId>a00fa931-0c27-430a-9f0a-ece82488a6cf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwczpcL1wvZGV2LXJlZ2lzdHJvZGVlbnZpb3Mub2x2YWNvdXJpZXIuY29tXC9vbHZhLWFnZW50ZXMtYXBpXC9wdWJsaWNcL2FwaVwvbG9naW4iLCJpYXQiOjE2OTE2OTU2MzgsImV4cCI6MTY5MTY5OTIzOCwibmJmIjoxNjkxNjk1NjM4LCJqdGkiOiJPY0hneVN0ZEJ2YlBrZDBNIiwic3ViIjpudWxsLCJwcnYiOiIyM2JkNWM4OTQ5ZjYwMGFkYjM5ZTcwMWM0MDA4NzJkYjdhNTk3NmY3In0.23OLPTAce6eacRNo8a7coBKmdjNx1dyVCVG0275UgMc</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;id\&quot;: 400374,\n\t\&quot;id_oficina\&quot;: \&quot;518\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>075933df-ff6b-4f60-a6d6-09fa93431606</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwczpcL1wvZGV2LXJlZ2lzdHJvZGVlbnZpb3Mub2x2YWNvdXJpZXIuY29tXC9vbHZhLWFnZW50ZXMtYXBpXC9wdWJsaWNcL2FwaVwvbG9naW4iLCJpYXQiOjE2OTE2OTU2MzgsImV4cCI6MTY5MTY5OTIzOCwibmJmIjoxNjkxNjk1NjM4LCJqdGkiOiJPY0hneVN0ZEJ2YlBrZDBNIiwic3ViIjpudWxsLCJwcnYiOiIyM2JkNWM4OTQ5ZjYwMGFkYjM5ZTcwMWM0MDA4NzJkYjdhNTk3NmY3In0.23OLPTAce6eacRNo8a7coBKmdjNx1dyVCVG0275UgMc</value>
      <webElementGuid>4305b9c0-eced-442e-b0db-8113a21fbb41</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dev-registrodeenvios.olvacourier.com/olva-agentes-api/public/api/recepcion</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
