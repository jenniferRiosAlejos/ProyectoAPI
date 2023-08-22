<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Listado de entregas</name>
   <tag></tag>
   <elementGuidId>07a632a5-4875-42db-b6fc-084111badfba</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwczpcL1wvZGV2LXJlZ2lzdHJvZGVlbnZpb3Mub2x2YWNvdXJpZXIuY29tXC9vbHZhLWFnZW50ZXMtYXBpXC9wdWJsaWNcL2FwaVwvbG9naW4iLCJpYXQiOjE2OTI2NTYwNzEsImV4cCI6MTcwODQyNDA3MSwibmJmIjoxNjkyNjU2MDcxLCJqdGkiOiJGc09lMGxNVEZ4eEp1clZOIiwic3ViIjpudWxsLCJwcnYiOiIyM2JkNWM4OTQ5ZjYwMGFkYjM5ZTcwMWM0MDA4NzJkYjdhNTk3NmY3In0.wIAr6jkdp_pl45N8yNfCt3qTcp6Mm6wkF-OomTtDjr0</value>
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
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwczpcL1wvZGV2LXJlZ2lzdHJvZGVlbnZpb3Mub2x2YWNvdXJpZXIuY29tXC9vbHZhLWFnZW50ZXMtYXBpXC9wdWJsaWNcL2FwaVwvbG9naW4iLCJpYXQiOjE2OTI2NTYwNzEsImV4cCI6MTcwODQyNDA3MSwibmJmIjoxNjkyNjU2MDcxLCJqdGkiOiJGc09lMGxNVEZ4eEp1clZOIiwic3ViIjpudWxsLCJwcnYiOiIyM2JkNWM4OTQ5ZjYwMGFkYjM5ZTcwMWM0MDA4NzJkYjdhNTk3NmY3In0.wIAr6jkdp_pl45N8yNfCt3qTcp6Mm6wkF-OomTtDjr0</value>
      <webElementGuid>63bc9ced-d449-4e09-b70f-322d5f7b917f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://dev-registrodeenvios.olvacourier.com/olva-agentes-api/public/api/entrega?id_oficina=518</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.tokenGlobal</defaultValue>
      <description></description>
      <id>05b92224-d104-4159-aa35-abf57ac3b46e</id>
      <masked>false</masked>
      <name>token1</name>
   </variables>
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
