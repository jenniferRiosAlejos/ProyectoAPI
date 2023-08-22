<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>RegistroConfirmación-Confirmación</name>
   <tag></tag>
   <elementGuidId>42771a81-0dff-42f8-909c-b0df4ef83079</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;ids\&quot;: [\n    88546262,\n\t88487692,\n    88407254\n  ],\n  \&quot;id_oficina\&quot;: 518\n}&quot;,
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
      <webElementGuid>c9793cf2-83ec-43ec-a39f-d38e1f25803d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwczpcL1wvZGV2LXJlZ2lzdHJvZGVlbnZpb3Mub2x2YWNvdXJpZXIuY29tXC9vbHZhLWFnZW50ZXMtYXBpXC9wdWJsaWNcL2FwaVwvbG9naW4iLCJpYXQiOjE2OTE2OTU2MzgsImV4cCI6MTY5MTY5OTIzOCwibmJmIjoxNjkxNjk1NjM4LCJqdGkiOiJPY0hneVN0ZEJ2YlBrZDBNIiwic3ViIjpudWxsLCJwcnYiOiIyM2JkNWM4OTQ5ZjYwMGFkYjM5ZTcwMWM0MDA4NzJkYjdhNTk3NmY3In0.23OLPTAce6eacRNo8a7coBKmdjNx1dyVCVG0275UgMc</value>
      <webElementGuid>31dea6da-2aa9-4d61-9f30-d8fda6ba302c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dev-registrodeenvios.olvacourier.com/olva-agentes-api/public/api/confirmar</restUrl>
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
