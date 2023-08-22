<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ObtenerDatosNroPedido</name>
   <tag></tag>
   <elementGuidId>b5f0b45e-494c-4a09-a7d2-eb5552d7a050</elementGuidId>
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
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwczpcL1wvZGV2LXJlZ2lzdHJvZGVlbnZpb3Mub2x2YWNvdXJpZXIuY29tXC9vbHZhLWFnZW50ZXMtYXBpXC9wdWJsaWNcL2FwaVwvbG9naW4iLCJpYXQiOjE2OTE2OTU2MzgsImV4cCI6MTY5MTY5OTIzOCwibmJmIjoxNjkxNjk1NjM4LCJqdGkiOiJPY0hneVN0ZEJ2YlBrZDBNIiwic3ViIjpudWxsLCJwcnYiOiIyM2JkNWM4OTQ5ZjYwMGFkYjM5ZTcwMWM0MDA4NzJkYjdhNTk3NmY3In0.23OLPTAce6eacRNo8a7coBKmdjNx1dyVCVG0275UgMc</value>
      <webElementGuid>117fe362-f4a0-44b3-8dcd-3382fab19bfe</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://dev-registrodeenvios.olvacourier.com/olva-agentes-api/public/api/recepcion/202000366107</restUrl>
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
